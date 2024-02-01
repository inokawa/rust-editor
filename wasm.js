/*! For license information please see wasm.js.LICENSE.txt */
(()=>{"use strict";var e,t,n,r,o,i,a,c,s,u,f,l,d,p,b,_,g,v,w,h,m,y,x,E,S,k,j,O,P,T,A,M,z,L,C={770:(e,t,n)=>{const r=Symbol("Comlink.proxy"),o=Symbol("Comlink.endpoint"),i=Symbol("Comlink.releaseProxy"),a=Symbol("Comlink.finalizer"),c=Symbol("Comlink.thrown"),s=e=>"object"==typeof e&&null!==e||"function"==typeof e,u=new Map([["proxy",{canHandle:e=>s(e)&&e[r],serialize(e){const{port1:t,port2:n}=new MessageChannel;return f(e,t),[n,[n]]},deserialize:e=>(e.start(),g(e,[],undefined))}],["throw",{canHandle:e=>s(e)&&c in e,serialize({value:e}){let t;return t=e instanceof Error?{isError:!0,value:{message:e.message,name:e.name,stack:e.stack}}:{isError:!1,value:e},[t,[]]},deserialize(e){if(e.isError)throw Object.assign(new Error(e.value.message),e.value);throw e.value}}]]);function f(e,t=globalThis,n=["*"]){t.addEventListener("message",(function o(i){if(!i||!i.data)return;if(!function(e,t){for(const n of e){if(t===n||"*"===n)return!0;if(n instanceof RegExp&&n.test(t))return!0}return!1}(n,i.origin))return void console.warn(`Invalid origin '${i.origin}' for comlink proxy`);const{id:s,type:u,path:d}=Object.assign({path:[]},i.data),p=(i.data.argumentList||[]).map(m);let b;try{const t=d.slice(0,-1).reduce(((e,t)=>e[t]),e),n=d.reduce(((e,t)=>e[t]),e);switch(u){case"GET":b=n;break;case"SET":t[d.slice(-1)[0]]=m(i.data.value),b=!0;break;case"APPLY":b=n.apply(t,p);break;case"CONSTRUCT":b=function(e){return Object.assign(e,{[r]:!0})}(new n(...p));break;case"ENDPOINT":{const{port1:t,port2:n}=new MessageChannel;f(e,n),b=function(e,t){return w.set(e,t),e}(t,[t])}break;case"RELEASE":b=void 0;break;default:return}}catch(e){b={value:e,[c]:0}}Promise.resolve(b).catch((e=>({value:e,[c]:0}))).then((n=>{const[r,i]=h(n);t.postMessage(Object.assign(Object.assign({},r),{id:s}),i),"RELEASE"===u&&(t.removeEventListener("message",o),l(t),a in e&&"function"==typeof e[a]&&e[a]())})).catch((e=>{const[n,r]=h({value:new TypeError("Unserializable return value"),[c]:0});t.postMessage(Object.assign(Object.assign({},n),{id:s}),r)}))})),t.start&&t.start()}function l(e){(function(e){return"MessagePort"===e.constructor.name})(e)&&e.close()}function d(e){if(e)throw new Error("Proxy has been released and is not useable")}function p(e){return y(e,{type:"RELEASE"}).then((()=>{l(e)}))}const b=new WeakMap,_="FinalizationRegistry"in globalThis&&new FinalizationRegistry((e=>{const t=(b.get(e)||0)-1;b.set(e,t),0===t&&p(e)}));function g(e,t=[],n=function(){}){let r=!1;const a=new Proxy(n,{get(n,o){if(d(r),o===i)return()=>{!function(e){_&&_.unregister(e)}(a),p(e),r=!0};if("then"===o){if(0===t.length)return{then:()=>a};const n=y(e,{type:"GET",path:t.map((e=>e.toString()))}).then(m);return n.then.bind(n)}return g(e,[...t,o])},set(n,o,i){d(r);const[a,c]=h(i);return y(e,{type:"SET",path:[...t,o].map((e=>e.toString())),value:a},c).then(m)},apply(n,i,a){d(r);const c=t[t.length-1];if(c===o)return y(e,{type:"ENDPOINT"}).then(m);if("bind"===c)return g(e,t.slice(0,-1));const[s,u]=v(a);return y(e,{type:"APPLY",path:t.map((e=>e.toString())),argumentList:s},u).then(m)},construct(n,o){d(r);const[i,a]=v(o);return y(e,{type:"CONSTRUCT",path:t.map((e=>e.toString())),argumentList:i},a).then(m)}});return function(e,t){const n=(b.get(t)||0)+1;b.set(t,n),_&&_.register(e,t,e)}(a,e),a}function v(e){const t=e.map(h);return[t.map((e=>e[0])),(n=t.map((e=>e[1])),Array.prototype.concat.apply([],n))];var n}const w=new WeakMap;function h(e){for(const[t,n]of u)if(n.canHandle(e)){const[r,o]=n.serialize(e);return[{type:"HANDLER",name:t,value:r},o]}return[{type:"RAW",value:e},w.get(e)||[]]}function m(e){switch(e.type){case"HANDLER":return u.get(e.name).deserialize(e.value);case"RAW":return e.value}}function y(e,t,n){return new Promise((r=>{const o=new Array(4).fill(0).map((()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16))).join("-");e.addEventListener("message",(function t(n){n.data&&n.data.id&&n.data.id===o&&(e.removeEventListener("message",t),r(n.data))})),e.start&&e.start(),e.postMessage(Object.assign({id:o},t),n)}))}var x=function(e,t,n,r){return new(n||(n=Promise))((function(o,i){function a(e){try{s(r.next(e))}catch(e){i(e)}}function c(e){try{s(r.throw(e))}catch(e){i(e)}}function s(e){var t;e.done?o(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(a,c)}s((r=r.apply(e,t||[])).next())}))},E=function(e,t){var n,r,o,i,a={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:c(0),throw:c(1),return:c(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function c(c){return function(s){return function(c){if(n)throw new TypeError("Generator is already executing.");for(;i&&(i=0,c[0]&&(a=0)),a;)try{if(n=1,r&&(o=2&c[0]?r.return:c[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,c[1])).done)return o;switch(r=0,o&&(c=[2&c[0],o.value]),c[0]){case 0:case 1:o=c;break;case 4:return a.label++,{value:c[1],done:!1};case 5:a.label++,r=c[1],c=[0];continue;case 7:c=a.ops.pop(),a.trys.pop();continue;default:if(!((o=(o=a.trys).length>0&&o[o.length-1])||6!==c[0]&&2!==c[0])){a=0;continue}if(3===c[0]&&(!o||c[1]>o[0]&&c[1]<o[3])){a.label=c[1];break}if(6===c[0]&&a.label<o[1]){a.label=o[1],o=c;break}if(o&&a.label<o[2]){a.label=o[2],a.ops.push(c);break}o[2]&&a.ops.pop(),a.trys.pop();continue}c=t.call(e,a)}catch(e){c=[6,e],r=0}finally{n=o=0}if(5&c[0])throw c[1];return{value:c[0]?c[1]:void 0,done:!0}}([c,s])}}},S=[];f({init:function(e,t,r){return x(void 0,void 0,void 0,(function(){return E(this,(function(o){switch(o.label){case 0:return self.term={write:e,read_key:function(){var e;return null===(e=S[0])||void 0===e?void 0:e[0]},read_ctrl:function(){var e,t;return null!==(t=null===(e=S[0])||void 0===e?void 0:e[1])&&void 0!==t&&t},read_end:function(){S.shift()},get_col_size:function(){return t},get_row_size:function(){return r}},[4,n.e(235).then(n.bind(n,235))];case 1:return o.sent(),[2]}}))}))},send_key:function(e,t){return x(void 0,void 0,void 0,(function(){return E(this,(function(n){return S.push([e,t]),[2]}))}))}})}},R={};function N(e){var t=R[e];if(void 0!==t)return t.exports;var n=R[e]={id:e,loaded:!1,exports:{}};return C[e](n,n.exports,N),n.loaded=!0,n.exports}N.m=C,N.c=R,N.d=(e,t)=>{for(var n in t)N.o(t,n)&&!N.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},N.f={},N.e=e=>Promise.all(Object.keys(N.f).reduce(((t,n)=>(N.f[n](e,t),t)),[])),N.u=e=>e+".js",N.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),N.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),N.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),N.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;N.g.importScripts&&(e=N.g.location+"");var t=N.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&!e;)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),N.p=e})(),(()=>{var e={872:1};N.f.i=(t,n)=>{e[t]||importScripts(N.p+N.u(t))};var t=self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[],n=t.push.bind(t);t.push=t=>{var[r,o,i]=t;for(var a in o)N.o(o,a)&&(N.m[a]=o[a]);for(i&&i(N);r.length;)e[r.pop()]=1;n(t)}})(),M={},z={373:function(){return{"./index_bg.js":{__wbg_getcolsize_b87b87997b69e977:function(){return void 0===e&&(e=N.c[508].exports),e.Jp()},__wbg_getrowsize_656445b44709ad42:function(){return void 0===t&&(t=N.c[508].exports),t.O0()},__wbindgen_cb_drop:function(e){return void 0===n&&(n=N.c[508].exports),n.G6(e)},__wbg_readkey_ac8d6e0854eee88e:function(e){return void 0===r&&(r=N.c[508].exports),r.t$(e)},__wbg_readctrl_9ad90fc4dc56da8a:function(){return void 0===o&&(o=N.c[508].exports),o.sQ()},__wbg_readend_90177d2b28069db6:function(){return void 0===i&&(i=N.c[508].exports),i.BM()},__wbg_write_d352b0c9b14ff07f:function(e,t){return void 0===a&&(a=N.c[508].exports),a.ne(e,t)},__wbindgen_object_drop_ref:function(e){return void 0===c&&(c=N.c[508].exports),c.ug(e)},__wbg_timeout_b77bcde72e171799:function(){return void 0===s&&(s=N.c[508].exports),s.fo()},__wbg_queueMicrotask_118eeb525d584d9a:function(e){return void 0===u&&(u=N.c[508].exports),u.er(e)},__wbg_queueMicrotask_26a89c14c53809c0:function(e){return void 0===f&&(f=N.c[508].exports),f.zd(e)},__wbindgen_is_function:function(e){return void 0===l&&(l=N.c[508].exports),l.o$(e)},__wbindgen_string_new:function(e,t){return void 0===d&&(d=N.c[508].exports),d.h4(e,t)},__wbg_now_65ff8ec2b863300c:function(e){return void 0===p&&(p=N.c[508].exports),p.DB(e)},__wbg_newnoargs_5859b6d41c6fe9f7:function(e,t){return void 0===b&&(b=N.c[508].exports),b.jl(e,t)},__wbg_get_5027b32da70f39b1:function(e,t){return void 0===_&&(_=N.c[508].exports),_.X8(e,t)},__wbg_call_a79f1973a4f07d5e:function(e,t){return void 0===g&&(g=N.c[508].exports),g.ly(e,t)},__wbindgen_object_clone_ref:function(e){return void 0===v&&(v=N.c[508].exports),v.m_(e)},__wbg_self_086b5302bcafb962:function(){return void 0===w&&(w=N.c[508].exports),w.Ph()},__wbg_window_132fa5d7546f1de5:function(){return void 0===h&&(h=N.c[508].exports),h.eo()},__wbg_globalThis_e5f801a37ad7d07b:function(){return void 0===m&&(m=N.c[508].exports),m.T9()},__wbg_global_f9a61fce4af6b7c1:function(){return void 0===y&&(y=N.c[508].exports),y.Od()},__wbindgen_is_undefined:function(e){return void 0===x&&(x=N.c[508].exports),x.XP(e)},__wbg_new_3a66822ed076951c:function(e,t){return void 0===E&&(E=N.c[508].exports),E.aj(e,t)},__wbg_resolve_97ecd55ee839391b:function(e){return void 0===S&&(S=N.c[508].exports),S.a7(e)},__wbg_then_7aeb7c5f1536640f:function(e,t){return void 0===k&&(k=N.c[508].exports),k.wh(e,t)},__wbg_then_5842e4e97f7beace:function(e,t,n){return void 0===j&&(j=N.c[508].exports),j.ps(e,t,n)},__wbindgen_debug_string:function(e,t){return void 0===O&&(O=N.c[508].exports),O.fY(e,t)},__wbindgen_throw:function(e,t){return void 0===P&&(P=N.c[508].exports),P.Or(e,t)},__wbindgen_rethrow:function(e){return void 0===T&&(T=N.c[508].exports),T.nD(e)},__wbindgen_closure_wrapper87:function(e,t,n){return void 0===A&&(A=N.c[508].exports),A.K(e,t,n)}}}}},L={235:[373]},N.w={},N.f.wasm=function(e,t){(L[e]||[]).forEach((function(n,r){var o=M[n];if(o)t.push(o);else{var i,a=z[n](),c=fetch(N.p+""+{235:{373:"cc5311cff543b001ebb4"}}[e][n]+".module.wasm");i=a&&"function"==typeof a.then&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(c),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(c,a):c.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)})),t.push(M[n]=i.then((function(e){return N.w[n]=(e.instance||e).exports})))}}))},N(770)})();