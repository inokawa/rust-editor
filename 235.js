"use strict";(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[235],{235:(n,e,t)=>{t.r(e),t.d(e,{__wbg_call_cb65541d95d71282:()=>_.Od,__wbg_get_97b561fb56f034b5:()=>_.To,__wbg_getcolsize_b87b87997b69e977:()=>_.Jp,__wbg_getrowsize_656445b44709ad42:()=>_.O0,__wbg_globalThis_1d39714405582d3c:()=>_.E$,__wbg_global_651f05c6a0944d1c:()=>_.c7,__wbg_new_d258248ed531ff54:()=>_.BM,__wbg_newnoargs_581967eacc0e2604:()=>_.$3,__wbg_now_0cfdc90c97d0c24b:()=>_.Z$,__wbg_readctrl_9ad90fc4dc56da8a:()=>_.sQ,__wbg_readend_90177d2b28069db6:()=>_.Li,__wbg_readkey_ac8d6e0854eee88e:()=>_.t$,__wbg_resolve_53698b95aaf7fcf8:()=>_.ot,__wbg_self_1ff1d729e9aae938:()=>_.ey,__wbg_set_wasm:()=>_.oT,__wbg_then_b2267541e2a73865:()=>_.w2,__wbg_then_f7e06ee3c11698eb:()=>_.vv,__wbg_timeout_b77bcde72e171799:()=>_.fo,__wbg_window_5f4faef6c12b79ec:()=>_.Qz,__wbg_write_d352b0c9b14ff07f:()=>_.ne,__wbindgen_cb_drop:()=>_.G6,__wbindgen_closure_wrapper76:()=>_.Yy,__wbindgen_debug_string:()=>_.fY,__wbindgen_is_undefined:()=>_.XP,__wbindgen_object_clone_ref:()=>_.m_,__wbindgen_object_drop_ref:()=>_.ug,__wbindgen_rethrow:()=>_.nD,__wbindgen_string_new:()=>_.h4,__wbindgen_throw:()=>_.Or,main_js:()=>_.NV});var r=t(373),_=t(508);(0,_.oT)(r),r.__wbindgen_start()},508:(n,e,t)=>{let r;function _(n){r=n}t.d(e,{Od:()=>Q,To:()=>P,Jp:()=>k,O0:()=>z,E$:()=>J,c7:()=>N,BM:()=>G,$3:()=>M,Z$:()=>L,sQ:()=>D,Li:()=>S,t$:()=>A,ot:()=>R,ey:()=>B,oT:()=>_,w2:()=>X,vv:()=>V,fo:()=>C,Qz:()=>I,ne:()=>j,G6:()=>E,Yy:()=>H,fY:()=>Z,XP:()=>q,m_:()=>Y,ug:()=>x,nD:()=>W,h4:()=>F,Or:()=>U,NV:()=>T}),n=t.hmd(n);const o=new Array(128).fill(void 0);function c(n){return o[n]}o.push(void 0,null,!0,!1);let i=o.length;function u(n){const e=c(n);return function(n){n<132||(o[n]=i,i=n)}(n),e}let f=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let l=null;function d(){return null!==l&&0!==l.byteLength||(l=new Uint8Array(r.memory.buffer)),l}function b(n,e){return n>>>=0,f.decode(d().subarray(n,n+e))}function a(n){i===o.length&&o.push(o.length+1);const e=i;return i=o[e],o[e]=n,e}function s(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=s(n[0]));for(let r=1;r<e;r++)t+=", "+s(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let g=0,w=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const m="function"==typeof w.encodeInto?function(n,e){return w.encodeInto(n,e)}:function(n,e){const t=w.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e,t){if(void 0===t){const t=w.encode(n),r=e(t.length,1)>>>0;return d().subarray(r,r+t.length).set(t),g=t.length,r}let r=n.length,_=e(r,1)>>>0;const o=d();let c=0;for(;c<r;c++){const e=n.charCodeAt(c);if(e>127)break;o[_+c]=e}if(c!==r){0!==c&&(n=n.slice(c)),_=t(_,r,r=c+3*n.length,1)>>>0;const e=d().subarray(_+c,_+r);c+=m(n,e).written}return g=c,_}let y=null;function p(){return null!==y&&0!==y.byteLength||(y=new Int32Array(r.memory.buffer)),y}function $(n,e,t){r._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h39ab4e4b8ffca803(n,e,a(t))}function v(n){return()=>{throw new Error(`${n} is not defined`)}}function T(){r.main_js()}function O(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(a(n))}}const k="function"==typeof term.get_col_size?term.get_col_size:v("term.get_col_size"),z="function"==typeof term.get_row_size?term.get_row_size:v("term.get_row_size");function x(n){u(n)}function E(n){const e=u(n).original;return 1==e.cnt--&&(e.a=0,!0)}function j(n,e){term.write(b(n,e))}function A(n){const e=term.read_key();var t=null==e?0:h(e,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t}const D="function"==typeof term.read_ctrl?term.read_ctrl:v("term.read_ctrl"),S="function"==typeof term.read_end?term.read_end:v("term.read_end");function C(){return a(new Promise((n=>setTimeout(n))))}function F(n,e){return a(b(n,e))}function L(n){return c(n).now()}function M(n,e){return a(new Function(b(n,e)))}function P(){return O((function(n,e){return a(Reflect.get(c(n),c(e)))}),arguments)}function Q(){return O((function(n,e){return a(c(n).call(c(e)))}),arguments)}function Y(n){return a(c(n))}function B(){return O((function(){return a(self.self)}),arguments)}function I(){return O((function(){return a(window.window)}),arguments)}function J(){return O((function(){return a(globalThis.globalThis)}),arguments)}function N(){return O((function(){return a(t.g.global)}),arguments)}function q(n){return void 0===c(n)}function G(n,e){return a(new Error(b(n,e)))}function R(n){return a(Promise.resolve(c(n)))}function V(n,e){return a(c(n).then(c(e)))}function X(n,e,t){return a(c(n).then(c(e),c(t)))}function Z(n,e){const t=h(s(c(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t}function U(n,e){throw new Error(b(n,e))}function W(n){throw u(n)}function H(n,e,t){const _=function(n,e,t,_){const o={a:n,b:e,cnt:1,dtor:15},c=(...n)=>{o.cnt++;const e=o.a;o.a=0;try{return _(e,o.b,...n)}finally{0==--o.cnt?r.__wbindgen_export_2.get(o.dtor)(e,o.b):o.a=e}};return c.original=o,c}(n,e,0,$);return a(_)}},373:(n,e,t)=>{var r=t.w[n.id];for(var _ in t.r(e),r)_&&(e[_]=r[_]);t(508),r[""]()}}]);