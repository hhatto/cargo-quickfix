"============================================================================
"File:        bro.vim
"Description: Syntax checking plugin for syntastic.vim
"Maintainer:  Justin Azoff <justin.azoff@gmail.com>
"License:     This program is free software. It comes without any warranty,
"             to the extent permitted by applicable law. You can redistribute
"             it and/or modify it under the terms of the Do What The Fuck You
"             Want To Public License, Version 2, as published by Sam Hocevar.
"             See http://sam.zoy.org/wtfpl/COPYING for more details.
"
"============================================================================

if exists('g:loaded_syntastic_rust_cargo_checker')
    finish
endif
let g:loaded_syntastic_rust_cargo_checker = 1

let s:save_cpo = &cpo
set cpo&vim

function! SyntaxCheckers_rust_cargo_GetLocList() dict
    let makeprg = self.makeprgBuild({ 'post_args': '' })

    let errorformat =
        \ '%f:%l:%c: %trror: %m,' .
        \ '%f:%l:%c: %tarning: %m'

    let env = syntastic#util#isRunningWindows() ? {} : { 'TERM': 'dumb' }

    return SyntasticMake({
        \ 'makeprg': makeprg,
        \ 'errorformat': errorformat,
        \ 'env': env })
endfunction

call g:SyntasticRegistry.CreateAndRegisterChecker({
    \ 'filetype': 'rust',
    \ 'name': 'cargo',
    \ 'exec': 'cargo-quickfix'})

let &cpo = s:save_cpo
unlet s:save_cpo

" vim: set sw=4 sts=4 et fdm=marker:
