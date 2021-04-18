#!/usr/bin/env python

import argparse
import os
import re

from function import Function
from function import read_functions

select_re = re.compile('LAPACK_(\w)_SELECT(\d)')


def is_scalar(name, cty, f):
    return (
        'c_char' in cty or
        name in [
            'abnrm',
            'abstol',
            'amax',
            'anorm',
            'bbnrm',
            'colcnd',
            'dif',
            'ihi',
            'il',
            'ilo',
            'info',
            'iter',
            'iu',
            'l',
            'liwork',
            'lrwork',
            'lwork',
            'm',
            'mm',
            'n',
            'n_err_bnds',
            'nb',
            'nrhs',
            'rank',
            'rcond',
            'rowcnd',
            'rpvgrw',
            'sdim',
            'tryrac',
            'vu',
        ] or
        name in [
            'alpha',
        ] and (
            'larfg' in f.name
        ) or
        name in [
            'p',
        ] and not (
            'tgevc' in f.name
        ) or
        name in [
            'q'
        ] and (
            'lapack_int' in cty
        ) or
        not (
            'geev' in f.name or
            'ggev' in f.name or
            'tgsna' in f.name or
            'trsna' in f.name
        )
        and name in [
            'vl',
            'vr',
        ] or
        name.startswith('inc') or
        name.startswith('k') or
        name.startswith('ld') or
        name.startswith('tol') or
        name.startswith('vers')
    )


def translate_argument(name, cty, f):
    m = select_re.match(cty)
    if m is not None:
        if m.group(1) == 'S':
            return 'Select{}F32'.format(m.group(2))
        elif m.group(1) == 'D':
            return 'Select{}F64'.format(m.group(2))
        elif m.group(1) == 'C':
            return 'Select{}C32'.format(m.group(2))
        elif m.group(1) == 'Z':
            return 'Select{}C64'.format(m.group(2))

    base = translate_type_base(cty)
    if '*const' in cty:
        if is_scalar(name, cty, f):
            return base
        else:
            return '&[{}]'.format(base)
    elif '*mut' in cty:
        if is_scalar(name, cty, f):
            return '&mut {}'.format(base)
        else:
            return '&mut [{}]'.format(base)

    return base


def translate_type_base(cty):
    cty = cty.replace('__BindgenComplex<f32>', 'lapack_complex_float')
    cty = cty.replace('__BindgenComplex<f64>', 'lapack_complex_double')
    cty = cty.replace('f32', 'c_float')
    cty = cty.replace('f64', 'c_double')

    if 'c_char' in cty:
        return 'u8'
    elif 'c_int' in cty:
        return 'i32'
    elif 'c_float' in cty:
        return 'f32'
    elif 'c_double' in cty:
        return 'f64'
    elif 'lapack_complex_float' in cty:
        return 'c32'
    elif 'lapack_complex_double' in cty:
        return 'c64'
    elif 'size_t' in cty:
        return 'size_t'

    assert False, 'cannot translate `{}`'.format(cty)


def translate_body_argument(name, rty):
    if rty.startswith('Select'):
        return 'transmute({})'.format(name)

    if rty == 'u8':
        return '&({} as c_char)'.format(name)
    elif rty == '&mut u8':
        return '{} as *mut _ as *mut _'.format(name)

    elif rty == 'i32':
        return '&{}'.format(name)
    elif rty == '&mut i32':
        return name
    elif rty == '&[i32]':
        return '{}.as_ptr()'.format(name)
    elif rty == '&mut [i32]':
        return '{}.as_mut_ptr()'.format(name)

    elif rty.startswith('f'):
        return '&{}'.format(name)
    elif rty.startswith('&mut f'):
        return name
    elif rty.startswith('&[f'):
        return '{}.as_ptr()'.format(name)
    elif rty.startswith('&mut [f'):
        return '{}.as_mut_ptr()'.format(name)

    elif rty.startswith('c'):
        return '&{} as *const _ as *const _'.format(name)
    elif rty.startswith('&mut c'):
        return '{} as *mut _ as *mut _'.format(name)
    elif rty.startswith('&[c'):
        return '{}.as_ptr() as *const _'.format(name)
    elif rty.startswith('&mut [c'):
        return '{}.as_mut_ptr() as *mut _'.format(name)

    elif rty == 'size_t':
        return name

    assert False, 'cannot translate `{}: {}`'.format(name, rty)


def translate_return_type(cty):
    cty = cty.replace('lapack_float_return', 'c_float')
    cty = cty.replace('f64', 'c_double')

    if cty == 'c_int':
        return 'i32'
    elif cty == 'c_float':
        return 'f32'
    elif cty == 'c_double':
        return 'f64'

    assert False, 'cannot translate `{}`'.format(cty)


def format_header(f):
    args = format_header_arguments(f)
    if f.ret is None:
        return 'pub unsafe fn {}({})'.format(f.name, args)
    else:
        return 'pub unsafe fn {}({}) -> {}'.format(f.name, args,
                                                   translate_return_type(f.ret))


def format_body(f):
    return 'ffi::{}_({})'.format(f.name, format_body_arguments(f))


def format_header_arguments(f):
    s = []
    for arg in f.args:
        s.append('{}: {}'.format(arg[0].lower(), translate_argument(*arg, f=f)))
    return ', '.join(s)


def format_body_arguments(f):
    s = []
    for arg in f.args:
        rty = translate_argument(*arg, f=f)
        s.append(translate_body_argument(arg[0].lower(), rty))
    return ', '.join(s)


def prepare(code):
    lines = filter(lambda line: not re.match(r'^\s*//.*', line),
                   code.split('\n'))
    lines = re.sub(r'\s+', ' ', ''.join(lines)).strip().split(';')
    lines = filter(lambda line: not re.match(r'^\s*$', line), lines)
    return [Function.parse(line) for line in lines]


def do(functions):
    for f in functions:
        if f.name in ['lsame']:
            continue
        print('\n#[inline]')
        print(format_header(f) + ' {')
        print('    ' + format_body(f) + '\n}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--sys', default='lapack-sys')
    arguments = parser.parse_args()
    path = os.path.join(arguments.sys, 'src', 'lapack.rs')
    do(prepare(read_functions(path)))
