(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[235],{235:(n,e,t)=>{"use strict";t.r(e),t.d(e,{__wbg_call_8487a9f580e47219:()=>o.AL,__wbg_get_a96a2f48856bb1c3:()=>o.fH,__wbg_getkey_17de46949815a52d:()=>o.Xl,__wbg_globalThis_a2669bee93faee43:()=>o.YB,__wbg_global_a5584d717f4d6761:()=>o.yJ,__wbg_newnoargs_179d393e4626fcf7:()=>o.vF,__wbg_now_9f22124bc74da886:()=>o.kj,__wbg_self_eeabd9085c04fc17:()=>o.cL,__wbg_setTimeout_b1970780692e9901:()=>o.XJ,__wbg_window_f110c13310da2c8f:()=>o.H5,__wbg_write_daa581465730aee1:()=>o.eV,__wbindgen_cb_drop:()=>o.G6,__wbindgen_closure_wrapper40:()=>o.MD,__wbindgen_debug_string:()=>o.fY,__wbindgen_is_undefined:()=>o.XP,__wbindgen_object_clone_ref:()=>o.m_,__wbindgen_object_drop_ref:()=>o.ug,__wbindgen_string_new:()=>o.h4,__wbindgen_throw:()=>o.Or,main_js:()=>o.NV});var r=t(716),o=t(838);r.__wbindgen_start()},838:(n,e,t)=>{"use strict";t.d(e,{NV:()=>k,eV:()=>T,Xl:()=>j,G6:()=>x,ug:()=>A,h4:()=>O,XJ:()=>$,kj:()=>F,fH:()=>X,AL:()=>D,m_:()=>E,vF:()=>J,cL:()=>S,H5:()=>C,YB:()=>H,yJ:()=>L,XP:()=>M,fY:()=>V,Or:()=>Y,MD:()=>B});var r=t(716);n=t.hmd(n);const o=new Array(32).fill(void 0);function _(n){return o[n]}o.push(void 0,null,!0,!1);let u=o.length;function c(n){const e=_(n);return function(n){n<36||(o[n]=u,u=n)}(n),e}let i=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let f=null;function l(){return null!==f&&f.buffer===r.memory.buffer||(f=new Uint8Array(r.memory.buffer)),f}function a(n,e){return i.decode(l().subarray(n,n+e))}function s(n){u===o.length&&o.push(o.length+1);const e=u;return u=o[e],o[e]=n,e}function b(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=b(n[0]));for(let r=1;r<e;r++)t+=", "+b(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let d=0,g=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const w="function"==typeof g.encodeInto?function(n,e){return g.encodeInto(n,e)}:function(n,e){const t=g.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e,t){if(void 0===t){const t=g.encode(n),r=e(t.length);return l().subarray(r,r+t.length).set(t),d=t.length,r}let r=n.length,o=e(r);const _=l();let u=0;for(;u<r;u++){const e=n.charCodeAt(u);if(e>127)break;_[o+u]=e}if(u!==r){0!==u&&(n=n.slice(u)),o=t(o,r,r=u+3*n.length);const e=l().subarray(o+u,o+r);u+=w(n,e).written}return d=u,o}let y=null;function m(){return null!==y&&y.buffer===r.memory.buffer||(y=new Int32Array(r.memory.buffer)),y}function p(n,e){r._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h77836a2cf3170ae1(n,e)}function k(){r.main_js()}function v(n){return function(){try{return n.apply(this,arguments)}catch(n){r.__wbindgen_exn_store(s(n))}}}const T=function(n,e){term.write(a(n,e))},j=function(n){var e=term.get_key(),t=null==e?0:h(e,r.__wbindgen_malloc,r.__wbindgen_realloc),o=d;m()[n/4+1]=o,m()[n/4+0]=t},x=function(n){const e=c(n).original;return 1==e.cnt--&&(e.a=0,!0)},A=function(n){c(n)},O=function(n,e){return s(a(n,e))},$=v((function(n,e,t){return _(n).setTimeout(_(e),t)})),F=function(n){return _(n).now()},X=v((function(n,e){return s(Reflect.get(_(n),_(e)))})),D=v((function(n,e){return s(_(n).call(_(e)))})),E=function(n){return s(_(n))},J=function(n,e){return s(new Function(a(n,e)))},S=v((function(){return s(self.self)})),C=v((function(){return s(window.window)})),H=v((function(){return s(globalThis.globalThis)})),L=v((function(){return s(t.g.global)})),M=function(n){return void 0===_(n)},V=function(n,e){var t=h(b(_(e)),r.__wbindgen_malloc,r.__wbindgen_realloc),o=d;m()[n/4+1]=o,m()[n/4+0]=t},Y=function(n,e){throw new Error(a(n,e))},B=function(n,e,t){return s(function(n,e,t,o){const _={a:n,b:e,cnt:1,dtor:10},u=(...n)=>{_.cnt++;const e=_.a;_.a=0;try{return o(e,_.b,...n)}finally{0==--_.cnt?r.__wbindgen_export_2.get(_.dtor)(e,_.b):_.a=e}};return u.original=_,u}(n,e,0,p))}},716:(n,e,t)=>{"use strict";var r=t.w[n.id];n.exports=r,t(838),r[""]()}}]);