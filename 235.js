"use strict";(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[235],{235:(n,e,t)=>{t.r(e),t.d(e,{__wbg_call_33d7bcddbbfa394a:()=>_.Jm,__wbg_get_72332cd2bc57924c:()=>_.NF,__wbg_getcolsize_a10523f79965babc:()=>_.$V,__wbg_getkey_17de46949815a52d:()=>_.Xl,__wbg_getrowsize_f108190ff429e4d8:()=>_.Qb,__wbg_globalThis_3348936ac49df00a:()=>_.lH,__wbg_global_67175caf56f55ca9:()=>_.qN,__wbg_new_3ee7ebe9952c1fbd:()=>_.UP,__wbg_newnoargs_971e9a5abe185139:()=>_._f,__wbg_now_c2563c77371d3ec4:()=>_.fU,__wbg_resolve_0107b3a501450ba0:()=>_.A8,__wbg_self_fd00a1ef86d1b2ed:()=>_.At,__wbg_then_18da6e5453572fc8:()=>_.LO,__wbg_then_e5489f796341454b:()=>_.dq,__wbg_timeout_6453e5d4636904c4:()=>_.sJ,__wbg_window_6f6e346d8bbd61d7:()=>_.Hc,__wbg_write_daa581465730aee1:()=>_.eV,__wbindgen_cb_drop:()=>_.G6,__wbindgen_closure_wrapper93:()=>_.gi,__wbindgen_debug_string:()=>_.fY,__wbindgen_is_undefined:()=>_.XP,__wbindgen_object_clone_ref:()=>_.m_,__wbindgen_object_drop_ref:()=>_.ug,__wbindgen_rethrow:()=>_.nD,__wbindgen_string_new:()=>_.h4,__wbindgen_throw:()=>_.Or,main_js:()=>_.NV});var r=t(373),_=t(508);r.__wbindgen_start()},508:(n,e,t)=>{t.d(e,{Jm:()=>U,NF:()=>P,$V:()=>x,Xl:()=>j,Qb:()=>O,lH:()=>S,qN:()=>C,UP:()=>I,_f:()=>F,fU:()=>q,A8:()=>L,At:()=>D,LO:()=>X,dq:()=>G,sJ:()=>E,Hc:()=>J,eV:()=>$,G6:()=>T,gi:()=>Y,fY:()=>M,XP:()=>H,m_:()=>V,ug:()=>z,nD:()=>R,h4:()=>N,Or:()=>Q,NV:()=>k});var r=t(373);n=t.hmd(n);const _=new Array(32).fill(void 0);function o(n){return _[n]}_.push(void 0,null,!0,!1);let i=_.length;function c(n){const e=o(n);return function(n){n<36||(_[n]=i,i=n)}(n),e}let u=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});u.decode();let f=new Uint8Array;function l(){return 0===f.byteLength&&(f=new Uint8Array(r.memory.buffer)),f}function a(n,e){return u.decode(l().subarray(n,n+e))}function s(n){i===_.length&&_.push(_.length+1);const e=i;return i=_[e],_[e]=n,e}function b(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=b(n[0]));for(let r=1;r<e;r++)t+=", "+b(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let g=0,d=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const w="function"==typeof d.encodeInto?function(n,e){return d.encodeInto(n,e)}:function(n,e){const t=d.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e,t){if(void 0===t){const t=d.encode(n),r=e(t.length);return l().subarray(r,r+t.length).set(t),g=t.length,r}let r=n.length,_=e(r);const o=l();let i=0;for(;i<r;i++){const e=n.charCodeAt(i);if(e>127)break;o[_+i]=e}if(i!==r){0!==i&&(n=n.slice(i)),_=t(_,r,r=i+3*n.length);const e=l().subarray(_+i,_+r);i+=w(n,e).written}return g=i,_}let m=new Int32Array;function y(){return 0===m.byteLength&&(m=new Int32Array(r.memory.buffer)),m}function p(n,e,t){r._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha0a371630a1ae826(n,e,s(t))}function A(n){return()=>{throw new Error(`${n} is not defined`)}}function k(){r.main_js()}function v(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(s(n))}}const x="function"==typeof term.get_col_size?term.get_col_size:A("term.get_col_size"),O="function"==typeof term.get_row_size?term.get_row_size:A("term.get_row_size");function T(n){const e=c(n).original;return 1==e.cnt--&&(e.a=0,!0)}function $(n,e){term.write(a(n,e))}function j(n){const e=term.get_key();var t=null==e?0:h(e,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;y()[n/4+1]=_,y()[n/4+0]=t}function z(n){c(n)}function E(){return s(new Promise((n=>setTimeout(n))))}function N(n,e){return s(a(n,e))}function q(n){return o(n).now()}function F(n,e){return s(new Function(a(n,e)))}function P(){return v((function(n,e){return s(Reflect.get(o(n),o(e)))}),arguments)}function U(){return v((function(n,e){return s(o(n).call(o(e)))}),arguments)}function V(n){return s(o(n))}function D(){return v((function(){return s(self.self)}),arguments)}function J(){return v((function(){return s(window.window)}),arguments)}function S(){return v((function(){return s(globalThis.globalThis)}),arguments)}function C(){return v((function(){return s(t.g.global)}),arguments)}function H(n){return void 0===o(n)}function I(n,e){return s(new Error(a(n,e)))}function L(n){return s(Promise.resolve(o(n)))}function X(n,e){return s(o(n).then(o(e)))}function G(n,e,t){return s(o(n).then(o(e),o(t)))}function M(n,e){const t=h(b(o(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;y()[n/4+1]=_,y()[n/4+0]=t}function Q(n,e){throw new Error(a(n,e))}function R(n){throw c(n)}function Y(n,e,t){const _=function(n,e,t,_){const o={a:n,b:e,cnt:1,dtor:28},i=(...n)=>{o.cnt++;const e=o.a;o.a=0;try{return _(e,o.b,...n)}finally{0==--o.cnt?r.__wbindgen_export_2.get(o.dtor)(e,o.b):o.a=e}};return i.original=o,i}(n,e,0,p);return s(_)}},373:(n,e,t)=>{var r=t.w[n.id];n.exports=r,t(508),r[""]()}}]);