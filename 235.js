(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[235],{235:(n,e,t)=>{"use strict";t.r(e),t.d(e,{__wbg_call_ba36642bd901572b:()=>o.qw,__wbg_get_800098c980b31ea2:()=>o.ol,__wbg_getkey_17de46949815a52d:()=>o.Xl,__wbg_globalThis_e0d21cabc6630763:()=>o.md,__wbg_global_8463719227271676:()=>o.IF,__wbg_newnoargs_9fdd8f3961dd1bee:()=>o.UL,__wbg_now_9f22124bc74da886:()=>o.kj,__wbg_self_bb69a836a72ec6e9:()=>o.tS,__wbg_setTimeout_b1970780692e9901:()=>o.XJ,__wbg_window_3304fc4b414c9693:()=>o.R$,__wbg_write_daa581465730aee1:()=>o.eV,__wbindgen_cb_drop:()=>o.G6,__wbindgen_closure_wrapper37:()=>o.h,__wbindgen_debug_string:()=>o.fY,__wbindgen_is_undefined:()=>o.XP,__wbindgen_object_clone_ref:()=>o.m_,__wbindgen_object_drop_ref:()=>o.ug,__wbindgen_string_new:()=>o.h4,__wbindgen_throw:()=>o.Or,main_js:()=>o.NV});var r=t(716),o=t(838);r.__wbindgen_start()},838:(n,e,t)=>{"use strict";t.d(e,{NV:()=>k,eV:()=>j,Xl:()=>v,G6:()=>x,ug:()=>$,h4:()=>O,XJ:()=>S,kj:()=>A,ol:()=>F,qw:()=>X,m_:()=>E,UL:()=>I,tS:()=>q,R$:()=>C,md:()=>R,IF:()=>V,XP:()=>D,fY:()=>J,Or:()=>N,h:()=>U});var r=t(716);n=t.hmd(n);const o=new Array(32).fill(void 0);function _(n){return o[n]}o.push(void 0,null,!0,!1);let u=o.length;function c(n){const e=_(n);return function(n){n<36||(o[n]=u,u=n)}(n),e}let i=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let f=null;function l(){return null!==f&&f.buffer===r.memory.buffer||(f=new Uint8Array(r.memory.buffer)),f}function a(n,e){return i.decode(l().subarray(n,n+e))}function b(n){u===o.length&&o.push(o.length+1);const e=u;return u=o[e],o[e]=n,e}function s(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=s(n[0]));for(let r=1;r<e;r++)t+=", "+s(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let d=0,g=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const w="function"==typeof g.encodeInto?function(n,e){return g.encodeInto(n,e)}:function(n,e){const t=g.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e,t){if(void 0===t){const t=g.encode(n),r=e(t.length);return l().subarray(r,r+t.length).set(t),d=t.length,r}let r=n.length,o=e(r);const _=l();let u=0;for(;u<r;u++){const e=n.charCodeAt(u);if(e>127)break;_[o+u]=e}if(u!==r){0!==u&&(n=n.slice(u)),o=t(o,r,r=u+3*n.length);const e=l().subarray(o+u,o+r);u+=w(n,e).written}return d=u,o}let m=null;function y(){return null!==m&&m.buffer===r.memory.buffer||(m=new Int32Array(r.memory.buffer)),m}function p(n,e){r._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf1c7c1acf26ddf36(n,e)}function k(){r.main_js()}function T(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(b(n))}}function j(n,e){term.write(a(n,e))}function v(n){var e=term.get_key(),t=null==e?0:h(e,r.__wbindgen_malloc,r.__wbindgen_realloc),o=d;y()[n/4+1]=o,y()[n/4+0]=t}function x(n){const e=c(n).original;return 1==e.cnt--&&(e.a=0,!0)}function $(n){c(n)}function O(n,e){return b(a(n,e))}function S(){return T((function(n,e,t){return _(n).setTimeout(_(e),t)}),arguments)}function A(n){return _(n).now()}function F(){return T((function(n,e){return b(Reflect.get(_(n),_(e)))}),arguments)}function X(){return T((function(n,e){return b(_(n).call(_(e)))}),arguments)}function E(n){return b(_(n))}function I(n,e){return b(new Function(a(n,e)))}function q(){return T((function(){return b(self.self)}),arguments)}function C(){return T((function(){return b(window.window)}),arguments)}function R(){return T((function(){return b(globalThis.globalThis)}),arguments)}function V(){return T((function(){return b(t.g.global)}),arguments)}function D(n){return void 0===_(n)}function J(n,e){var t=h(s(_(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),o=d;y()[n/4+1]=o,y()[n/4+0]=t}function N(n,e){throw new Error(a(n,e))}function U(n,e,t){return b(function(n,e,t,o){const _={a:n,b:e,cnt:1,dtor:7},u=(...n)=>{_.cnt++;const e=_.a;_.a=0;try{return o(e,_.b,...n)}finally{0==--_.cnt?r.__wbindgen_export_2.get(_.dtor)(e,_.b):_.a=e}};return u.original=_,u}(n,e,0,p))}},716:(n,e,t)=>{"use strict";var r=t.w[n.id];n.exports=r,t(838),r[""]()}}]);