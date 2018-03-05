#!/usr/bin/env python

from function import Function, read_functions
import argparse
import os
import re

select_re = re.compile('LAPACK_(\w)_SELECT(\d)')

def is_const(name, cty):
    return '*const' in cty

def is_mut(name, cty):
    return '*mut' in cty

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
        name == 'q' and 'lapack_int' in cty or
        not (
            'geev' in f.name or
            'tgsna' in f.name or
            'trsna' in f.name
        ) and name in [
            'vl',
            'vr',
        ] or
        not ('tgevc' in f.name) and name in [
            'p',
        ] or
        name.startswith('alpha') or
        name.startswith('beta') or
        name.startswith('inc') or
        name.startswith('k') or
        name.startswith('ld') or
        name.startswith('tol') or
        name.startswith('vers')
    )

def translate_argument(name, cty, f):
    if is_const(name, cty):
        base = translate_type_base(cty, f)
        if is_scalar(name, cty, f):
            return base
        else:
            return '&[{}]'.format(base)
    elif is_mut(name, cty):
        base = translate_type_base(cty, f)
        if is_scalar(name, cty, f):
            return '&mut {}'.format(base)
        else:
            return '&mut [{}]'.format(base)

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

    assert False, 'cannot translate `{}: {}`'.format(name, cty)

def translate_type_base(cty, f):
    if 'c_char' in cty:
        return 'u8'
    elif 'lapack_int' in cty or 'lapack_logical' in cty:
        return 'i32'
    elif 'lapack_complex_double' in cty:
        return 'c64'
    elif 'lapack_complex_float' in cty:
        return 'c32'
    elif 'c_double' in cty:
        return 'f64'
    elif 'c_float' in cty:
        return 'f32'

    assert False, 'cannot translate `{}` in `{}`'.format(cty, f.name)

def translate_body_argument(name, rty):
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
        return '{}'.format(name)
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

    if rty.startswith('Select'):
        return 'transmute({})'.format(name)

    assert False, 'cannot translate `{}: {}`'.format(name, rty)

def translate_return_type(cty):
    if cty == 'c_float':
        return 'f32'
    elif cty == 'c_double':
        return 'f64'

    assert False, 'cannot translate `{}`'.format(cty)

def format_header(f):
    args = format_header_arguments(f)
    if f.ret is None:
        return 'pub unsafe fn {}({})'.format(f.name, args)
    else:
        return 'pub unsafe fn {}({}) -> {}'.format(f.name, args, translate_return_type(f.ret))

def format_body(f):
    return 'ffi::{}_({})'.format(f.name, format_body_arguments(f))

def format_header_arguments(f):
    s = []
    for arg in f.args:
        s.append('{}: {}'.format(arg[0], translate_argument(*arg, f=f)))
    return ', '.join(s)

def format_body_arguments(f):
    s = []
    for arg in f.args:
        rty = translate_argument(*arg, f=f)
        s.append(translate_body_argument(arg[0], rty))
    return ', '.join(s)

def prepare(code):
    lines = filter(lambda line: not re.match(r'^\s*//.*', line), code.split('\n'))
    lines = re.sub(r'\s+', ' ', ''.join(lines)).strip().split(';')
    lines = filter(lambda line: not re.match(r'^\s*$', line), lines)
    return [Function.parse(line) for line in lines]

def do(functions):
    for f in functions:
        print('\n#[inline]')
        print(format_header(f) + ' {')
        print('    ' + format_body(f) + '\n}')

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--sys', required=True)
    arguments = parser.parse_args()
    path = os.path.join(arguments.sys, 'src', 'lib.rs')
    do(prepare(read_functions(path)))
