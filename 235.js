(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[235],{235:(n,e,t)=>{"use strict";t.r(e),t.d(e,{__wbg_call_89558c3e96703ca1:()=>_.Z4,__wbg_get_8bbb82393651dd9c:()=>_.gM,__wbg_getcolsize_a10523f79965babc:()=>_.$V,__wbg_getkey_17de46949815a52d:()=>_.Xl,__wbg_getrowsize_f108190ff429e4d8:()=>_.Qb,__wbg_globalThis_d61b1f48a57191ae:()=>_.EB,__wbg_global_e7669da72fd7f239:()=>_.Yc,__wbg_new_55259b13834a484c:()=>_.lQ,__wbg_newnoargs_f579424187aa1717:()=>_.bf,__wbg_now_44a034aa2e1d73dd:()=>_.OK,__wbg_resolve_4f8f547f26b30b27:()=>_.Yp,__wbg_self_e23d74ae45fb17d1:()=>_.tL,__wbg_then_58a04e42527f52c6:()=>_.YI,__wbg_then_a6860c82b90816ca:()=>_.wW,__wbg_timeout_6453e5d4636904c4:()=>_.sJ,__wbg_window_b4be7f48b24ac56e:()=>_.Qu,__wbg_write_daa581465730aee1:()=>_.eV,__wbindgen_cb_drop:()=>_.G6,__wbindgen_closure_wrapper93:()=>_.gi,__wbindgen_debug_string:()=>_.fY,__wbindgen_is_undefined:()=>_.XP,__wbindgen_object_clone_ref:()=>_.m_,__wbindgen_object_drop_ref:()=>_.ug,__wbindgen_rethrow:()=>_.nD,__wbindgen_string_new:()=>_.h4,__wbindgen_throw:()=>_.Or,main_js:()=>_.NV});var r=t(716),_=t(508);r.__wbindgen_start()},508:(n,e,t)=>{"use strict";t.d(e,{Z4:()=>I,gM:()=>D,$V:()=>O,Xl:()=>z,Qb:()=>T,EB:()=>J,Yc:()=>N,lQ:()=>C,bf:()=>V,OK:()=>Q,Yp:()=>F,tL:()=>X,YI:()=>P,wW:()=>M,sJ:()=>$,Qu:()=>B,eV:()=>j,G6:()=>E,gi:()=>L,fY:()=>q,XP:()=>W,m_:()=>S,ug:()=>Y,nD:()=>K,h4:()=>A,Or:()=>G,NV:()=>v});var r=t(716);n=t.hmd(n);const _=new Array(32).fill(void 0);function o(n){return _[n]}_.push(void 0,null,!0,!1);let u=_.length;function i(n){const e=o(n);return function(n){n<36||(_[n]=u,u=n)}(n),e}let c=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});c.decode();let f=null;function l(){return null!==f&&f.buffer===r.memory.buffer||(f=new Uint8Array(r.memory.buffer)),f}function b(n,e){return c.decode(l().subarray(n,n+e))}function a(n){u===_.length&&_.push(_.length+1);const e=u;return u=_[e],_[e]=n,e}function s(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=s(n[0]));for(let r=1;r<e;r++)t+=", "+s(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let g=0,w=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const d="function"==typeof w.encodeInto?function(n,e){return w.encodeInto(n,e)}:function(n,e){const t=w.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e,t){if(void 0===t){const t=w.encode(n),r=e(t.length);return l().subarray(r,r+t.length).set(t),g=t.length,r}let r=n.length,_=e(r);const o=l();let u=0;for(;u<r;u++){const e=n.charCodeAt(u);if(e>127)break;o[_+u]=e}if(u!==r){0!==u&&(n=n.slice(u)),_=t(_,r,r=u+3*n.length);const e=l().subarray(_+u,_+r);u+=d(n,e).written}return g=u,_}let m=null;function y(){return null!==m&&m.buffer===r.memory.buffer||(m=new Int32Array(r.memory.buffer)),m}function p(n,e,t){r._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h04154439a4643074(n,e,a(t))}function v(){r.main_js()}function k(n){return()=>{throw new Error(`${n} is not defined`)}}function x(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(a(n))}}const O="function"==typeof term.get_col_size?term.get_col_size:k("term.get_col_size"),T="function"==typeof term.get_row_size?term.get_row_size:k("term.get_row_size");function $(){return a(new Promise((n=>setTimeout(n))))}function E(n){const e=i(n).original;return 1==e.cnt--&&(e.a=0,!0)}function j(n,e){term.write(b(n,e))}function z(n){var e=term.get_key(),t=null==e?0:h(e,r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;y()[n/4+1]=_,y()[n/4+0]=t}function Y(n){i(n)}function A(n,e){return a(b(n,e))}function Q(n){return o(n).now()}function V(n,e){return a(new Function(b(n,e)))}function D(){return x((function(n,e){return a(Reflect.get(o(n),o(e)))}),arguments)}function I(){return x((function(n,e){return a(o(n).call(o(e)))}),arguments)}function S(n){return a(o(n))}function C(n,e){return a(new Error(b(n,e)))}function F(n){return a(Promise.resolve(o(n)))}function M(n,e){return a(o(n).then(o(e)))}function P(n,e,t){return a(o(n).then(o(e),o(t)))}function X(){return x((function(){return a(self.self)}),arguments)}function B(){return x((function(){return a(window.window)}),arguments)}function J(){return x((function(){return a(globalThis.globalThis)}),arguments)}function N(){return x((function(){return a(t.g.global)}),arguments)}function W(n){return void 0===o(n)}function q(n,e){var t=h(s(o(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),_=g;y()[n/4+1]=_,y()[n/4+0]=t}function G(n,e){throw new Error(b(n,e))}function K(n){throw i(n)}function L(n,e,t){return a(function(n,e,t,_){const o={a:n,b:e,cnt:1,dtor:30},u=(...n)=>{o.cnt++;const e=o.a;o.a=0;try{return _(e,o.b,...n)}finally{0==--o.cnt?r.__wbindgen_export_2.get(o.dtor)(e,o.b):o.a=e}};return u.original=o,u}(n,e,0,p))}},716:(n,e,t)=>{"use strict";var r=t.w[n.id];n.exports=r,t(508),r[""]()}}]);