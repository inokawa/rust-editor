"use strict";(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[300],{300:(n,e,t)=>{t.r(e),t.d(e,{__wbg_call_27c0f87801dedf93:()=>_.cq,__wbg_get_e3c254076557e348:()=>_.XB,__wbg_getcolsize_b87b87997b69e977:()=>_.WY,__wbg_getrowsize_656445b44709ad42:()=>_.TF,__wbg_globalThis_d1e6af4856ba331b:()=>_.Kc,__wbg_global_207b558942527489:()=>_.vA,__wbg_new_28c511d9baebfa89:()=>_.mC,__wbg_newnoargs_e258087cd0daa0ea:()=>_.Pf,__wbg_now_4e659b3d15f470d9:()=>_.xZ,__wbg_queueMicrotask_3cbae2ec6b6cd3d6:()=>_.Fq,__wbg_queueMicrotask_481971b0d87f3dd4:()=>_.fi,__wbg_readctrl_9ad90fc4dc56da8a:()=>_.wF,__wbg_readend_90177d2b28069db6:()=>_.J_,__wbg_readkey_ac8d6e0854eee88e:()=>_.Yb,__wbg_resolve_b0083a7967828ec8:()=>_.w,__wbg_self_ce0dbfc45cf2f5be:()=>_.cX,__wbg_set_wasm:()=>_.lI,__wbg_then_0c86a60e8fcfe9f6:()=>_.MG,__wbg_then_a73caa9a87991566:()=>_.bs,__wbg_timeout_b77bcde72e171799:()=>_.Ic,__wbg_window_c6fb939a7f436783:()=>_.kh,__wbg_write_d352b0c9b14ff07f:()=>_.oh,__wbindgen_cb_drop:()=>_.LC,__wbindgen_closure_wrapper88:()=>_.VM,__wbindgen_debug_string:()=>_.rl,__wbindgen_is_function:()=>_.PR,__wbindgen_is_undefined:()=>_.vU,__wbindgen_object_clone_ref:()=>_.BZ,__wbindgen_object_drop_ref:()=>_.bk,__wbindgen_rethrow:()=>_.PG,__wbindgen_string_new:()=>_.yc,__wbindgen_throw:()=>_.Qn,main_js:()=>_.GT});var r=t(923),_=t(614);(0,_.lI)(r),r.__wbindgen_start()},614:(n,e,t)=>{let r;function _(n){r=n}t.d(e,{cq:()=>X,XB:()=>L,WY:()=>q,TF:()=>z,Kc:()=>J,vA:()=>U,mC:()=>K,Pf:()=>S,xZ:()=>B,Fq:()=>G,fi:()=>E,wF:()=>I,J_:()=>P,Yb:()=>C,w:()=>Q,cX:()=>Z,lI:()=>_,MG:()=>V,bs:()=>N,Ic:()=>$,kh:()=>D,oh:()=>j,LC:()=>A,VM:()=>tn,rl:()=>H,PR:()=>O,vU:()=>W,BZ:()=>Y,bk:()=>M,PG:()=>en,yc:()=>R,Qn:()=>nn,GT:()=>x}),n=t.hmd(n);const o=new Array(128).fill(void 0);function c(n){return o[n]}o.push(void 0,null,!0,!1);let i=o.length;function u(n){const e=c(n);return function(n){n<132||(o[n]=i,i=n)}(n),e}let f=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let l=null;function b(){return null!==l&&0!==l.byteLength||(l=new Uint8Array(r.memory.buffer)),l}function a(n,e){return n>>>=0,f.decode(b().subarray(n,n+e))}function d(n){i===o.length&&o.push(o.length+1);const e=i;return i=o[e],o[e]=n,e}function s(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=s(n[0]));for(let r=1;r<e;r++)t+=", "+s(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let g=0,w=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof w.encodeInto?function(n,e){return w.encodeInto(n,e)}:function(n,e){const t=w.encode(n);return e.set(t),{read:n.length,written:t.length}};function m(n,e,t){if(void 0===t){const t=w.encode(n),r=e(t.length,1)>>>0;return b().subarray(r,r+t.length).set(t),g=t.length,r}let r=n.length,_=e(r,1)>>>0;const o=b();let c=0;for(;c<r;c++){const e=n.charCodeAt(c);if(e>127)break;o[_+c]=e}if(c!==r){0!==c&&(n=n.slice(c)),_=t(_,r,r=c+3*n.length,1)>>>0;const e=b().subarray(_+c,_+r);c+=h(n,e).written,_=t(_,r,c,1)>>>0}return g=c,_}let y=null;function p(){return null!==y&&0!==y.byteLength||(y=new Int32Array(r.memory.buffer)),y}const k="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((n=>{r.__wbindgen_export_2.get(n.dtor)(n.a,n.b)}));function v(n,e,t){r._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h729b971d187999ec(n,e,d(t))}function T(n){return()=>{throw new Error(`${n} is not defined`)}}function x(){r.main_js()}function F(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(d(n))}}const q="function"==typeof term.get_col_size?term.get_col_size:T("term.get_col_size"),z="function"==typeof term.get_row_size?term.get_row_size:T("term.get_row_size");function M(n){u(n)}function A(n){const e=u(n).original;return 1==e.cnt--&&(e.a=0,!0)}function j(n,e){term.write(a(n,e))}function C(n){const e=term.read_key();var t=null==e?0:m(e,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t}const I="function"==typeof term.read_ctrl?term.read_ctrl:T("term.read_ctrl"),P="function"==typeof term.read_end?term.read_end:T("term.read_end");function $(){return d(new Promise((n=>setTimeout(n))))}function E(n){queueMicrotask(c(n))}function G(n){return d(c(n).queueMicrotask)}function O(n){return"function"==typeof c(n)}function R(n,e){return d(a(n,e))}function B(n){return c(n).now()}function S(n,e){return d(new Function(a(n,e)))}function L(){return F((function(n,e){return d(Reflect.get(c(n),c(e)))}),arguments)}function X(){return F((function(n,e){return d(c(n).call(c(e)))}),arguments)}function Y(n){return d(c(n))}function Z(){return F((function(){return d(self.self)}),arguments)}function D(){return F((function(){return d(window.window)}),arguments)}function J(){return F((function(){return d(globalThis.globalThis)}),arguments)}function U(){return F((function(){return d(t.g.global)}),arguments)}function W(n){return void 0===c(n)}function K(n,e){return d(new Error(a(n,e)))}function Q(n){return d(Promise.resolve(c(n)))}function V(n,e){return d(c(n).then(c(e)))}function N(n,e,t){return d(c(n).then(c(e),c(t)))}function H(n,e){const t=m(s(c(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;p()[n/4+1]=_,p()[n/4+0]=t}function nn(n,e){throw new Error(a(n,e))}function en(n){throw u(n)}function tn(n,e,t){const _=function(n,e,t,_){const o={a:n,b:e,cnt:1,dtor:17},c=(...n)=>{o.cnt++;const e=o.a;o.a=0;try{return _(e,o.b,...n)}finally{0==--o.cnt?(r.__wbindgen_export_2.get(o.dtor)(e,o.b),k.unregister(o)):o.a=e}};return c.original=o,k.register(c,o,o),c}(n,e,0,v);return d(_)}},923:(n,e,t)=>{var r=t.w[n.id];for(var _ in t.r(e),r)_&&(e[_]=r[_]);t(614),r[""]()}}]);