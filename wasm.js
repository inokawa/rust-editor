/*! For license information please see wasm.js.LICENSE.txt */
(()=>{"use strict";var e,t,n,r,o,i,a,s,c,u,f,l,p,d,b,_,g,h,w,v,m,y,x,E,S,k,O,j,P,T={770:(e,t,n)=>{const r=Symbol("Comlink.proxy"),o=Symbol("Comlink.endpoint"),i=Symbol("Comlink.releaseProxy"),a=Symbol("Comlink.finalizer"),s=Symbol("Comlink.thrown"),c=e=>"object"==typeof e&&null!==e||"function"==typeof e,u=new Map([["proxy",{canHandle:e=>c(e)&&e[r],serialize(e){const{port1:t,port2:n}=new MessageChannel;return f(e,t),[n,[n]]},deserialize:e=>(e.start(),g(e,[],undefined))}],["throw",{canHandle:e=>c(e)&&s in e,serialize({value:e}){let t;return t=e instanceof Error?{isError:!0,value:{message:e.message,name:e.name,stack:e.stack}}:{isError:!1,value:e},[t,[]]},deserialize(e){if(e.isError)throw Object.assign(new Error(e.value.message),e.value);throw e.value}}]]);function f(e,t=globalThis,n=["*"]){t.addEventListener("message",(function o(i){if(!i||!i.data)return;if(!function(e,t){for(const n of e){if(t===n||"*"===n)return!0;if(n instanceof RegExp&&n.test(t))return!0}return!1}(n,i.origin))return void console.warn(`Invalid origin '${i.origin}' for comlink proxy`);const{id:c,type:u,path:p}=Object.assign({path:[]},i.data),d=(i.data.argumentList||[]).map(m);let b;try{const t=p.slice(0,-1).reduce(((e,t)=>e[t]),e),n=p.reduce(((e,t)=>e[t]),e);switch(u){case"GET":b=n;break;case"SET":t[p.slice(-1)[0]]=m(i.data.value),b=!0;break;case"APPLY":b=n.apply(t,d);break;case"CONSTRUCT":b=function(e){return Object.assign(e,{[r]:!0})}(new n(...d));break;case"ENDPOINT":{const{port1:t,port2:n}=new MessageChannel;f(e,n),b=function(e,t){return w.set(e,t),e}(t,[t])}break;case"RELEASE":b=void 0;break;default:return}}catch(e){b={value:e,[s]:0}}Promise.resolve(b).catch((e=>({value:e,[s]:0}))).then((n=>{const[r,i]=v(n);t.postMessage(Object.assign(Object.assign({},r),{id:c}),i),"RELEASE"===u&&(t.removeEventListener("message",o),l(t),a in e&&"function"==typeof e[a]&&e[a]())})).catch((e=>{const[n,r]=v({value:new TypeError("Unserializable return value"),[s]:0});t.postMessage(Object.assign(Object.assign({},n),{id:c}),r)}))})),t.start&&t.start()}function l(e){(function(e){return"MessagePort"===e.constructor.name})(e)&&e.close()}function p(e){if(e)throw new Error("Proxy has been released and is not useable")}function d(e){return y(e,{type:"RELEASE"}).then((()=>{l(e)}))}const b=new WeakMap,_="FinalizationRegistry"in globalThis&&new FinalizationRegistry((e=>{const t=(b.get(e)||0)-1;b.set(e,t),0===t&&d(e)}));function g(e,t=[],n=function(){}){let r=!1;const a=new Proxy(n,{get(n,o){if(p(r),o===i)return()=>{!function(e){_&&_.unregister(e)}(a),d(e),r=!0};if("then"===o){if(0===t.length)return{then:()=>a};const n=y(e,{type:"GET",path:t.map((e=>e.toString()))}).then(m);return n.then.bind(n)}return g(e,[...t,o])},set(n,o,i){p(r);const[a,s]=v(i);return y(e,{type:"SET",path:[...t,o].map((e=>e.toString())),value:a},s).then(m)},apply(n,i,a){p(r);const s=t[t.length-1];if(s===o)return y(e,{type:"ENDPOINT"}).then(m);if("bind"===s)return g(e,t.slice(0,-1));const[c,u]=h(a);return y(e,{type:"APPLY",path:t.map((e=>e.toString())),argumentList:c},u).then(m)},construct(n,o){p(r);const[i,a]=h(o);return y(e,{type:"CONSTRUCT",path:t.map((e=>e.toString())),argumentList:i},a).then(m)}});return function(e,t){const n=(b.get(t)||0)+1;b.set(t,n),_&&_.register(e,t,e)}(a,e),a}function h(e){const t=e.map(v);return[t.map((e=>e[0])),(n=t.map((e=>e[1])),Array.prototype.concat.apply([],n))];var n}const w=new WeakMap;function v(e){for(const[t,n]of u)if(n.canHandle(e)){const[r,o]=n.serialize(e);return[{type:"HANDLER",name:t,value:r},o]}return[{type:"RAW",value:e},w.get(e)||[]]}function m(e){switch(e.type){case"HANDLER":return u.get(e.name).deserialize(e.value);case"RAW":return e.value}}function y(e,t,n){return new Promise((r=>{const o=new Array(4).fill(0).map((()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16))).join("-");e.addEventListener("message",(function t(n){n.data&&n.data.id&&n.data.id===o&&(e.removeEventListener("message",t),r(n.data))})),e.start&&e.start(),e.postMessage(Object.assign({id:o},t),n)}))}var x=function(e,t,n,r){return new(n||(n=Promise))((function(o,i){function a(e){try{c(r.next(e))}catch(e){i(e)}}function s(e){try{c(r.throw(e))}catch(e){i(e)}}function c(e){var t;e.done?o(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(a,s)}c((r=r.apply(e,t||[])).next())}))},E=function(e,t){var n,r,o,i,a={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:s(0),throw:s(1),return:s(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function s(s){return function(c){return function(s){if(n)throw new TypeError("Generator is already executing.");for(;i&&(i=0,s[0]&&(a=0)),a;)try{if(n=1,r&&(o=2&s[0]?r.return:s[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,s[1])).done)return o;switch(r=0,o&&(s=[2&s[0],o.value]),s[0]){case 0:case 1:o=s;break;case 4:return a.label++,{value:s[1],done:!1};case 5:a.label++,r=s[1],s=[0];continue;case 7:s=a.ops.pop(),a.trys.pop();continue;default:if(!((o=(o=a.trys).length>0&&o[o.length-1])||6!==s[0]&&2!==s[0])){a=0;continue}if(3===s[0]&&(!o||s[1]>o[0]&&s[1]<o[3])){a.label=s[1];break}if(6===s[0]&&a.label<o[1]){a.label=o[1],o=s;break}if(o&&a.label<o[2]){a.label=o[2],a.ops.push(s);break}o[2]&&a.ops.pop(),a.trys.pop();continue}s=t.call(e,a)}catch(e){s=[6,e],r=0}finally{n=o=0}if(5&s[0])throw s[1];return{value:s[0]?s[1]:void 0,done:!0}}([s,c])}}},S=[];f({init:function(e,t,r){return x(void 0,void 0,void 0,(function(){return E(this,(function(o){switch(o.label){case 0:return self.term={write:e,get_key:function(){var e=S.shift();return null==e?void 0:e.key},get_col_size:function(){return t},get_row_size:function(){return r}},[4,n.e(235).then(n.bind(n,235))];case 1:return o.sent(),[2]}}))}))},send_key:function(e,t){return x(void 0,void 0,void 0,(function(){return E(this,(function(n){return S.push({key:e,ctrl:t}),[2]}))}))}})}},A={};function M(e){var t=A[e];if(void 0!==t)return t.exports;var n=A[e]={id:e,loaded:!1,exports:{}};return T[e](n,n.exports,M),n.loaded=!0,n.exports}M.m=T,M.c=A,M.d=(e,t)=>{for(var n in t)M.o(t,n)&&!M.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},M.f={},M.e=e=>Promise.all(Object.keys(M.f).reduce(((t,n)=>(M.f[n](e,t),t)),[])),M.u=e=>e+".js",M.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),M.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),M.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),M.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;M.g.importScripts&&(e=M.g.location+"");var t=M.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");n.length&&(e=n[n.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),M.p=e})(),(()=>{var e={872:1};M.f.i=(t,n)=>{e[t]||importScripts(M.p+M.u(t))};var t=self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[],n=t.push.bind(t);t.push=t=>{var[r,o,i]=t;for(var a in o)M.o(o,a)&&(M.m[a]=o[a]);for(i&&i(M);r.length;)e[r.pop()]=1;n(t)}})(),O={},j={373:function(){return{"./index_bg.js":{__wbg_getcolsize_b87b87997b69e977:function(){return void 0===e&&(e=M.c[508].exports),e.Jp()},__wbg_getrowsize_656445b44709ad42:function(){return void 0===t&&(t=M.c[508].exports),t.O0()},__wbindgen_cb_drop:function(e){return void 0===n&&(n=M.c[508].exports),n.G6(e)},__wbg_getkey_6bf2dced1f144f64:function(e){return void 0===r&&(r=M.c[508].exports),r.Ok(e)},__wbg_write_d352b0c9b14ff07f:function(e,t){return void 0===o&&(o=M.c[508].exports),o.ne(e,t)},__wbindgen_object_drop_ref:function(e){return void 0===i&&(i=M.c[508].exports),i.ug(e)},__wbg_timeout_b77bcde72e171799:function(){return void 0===a&&(a=M.c[508].exports),a.fo()},__wbindgen_string_new:function(e,t){return void 0===s&&(s=M.c[508].exports),s.h4(e,t)},__wbg_now_c644db5194be8437:function(e){return void 0===c&&(c=M.c[508].exports),c.LR(e)},__wbg_newnoargs_2b8b6bd7753c76ba:function(e,t){return void 0===u&&(u=M.c[508].exports),u.Ru(e,t)},__wbg_get_baf4855f9a986186:function(e,t){return void 0===f&&(f=M.c[508].exports),f.Qk(e,t)},__wbg_call_95d1ea488d03e4e8:function(e,t){return void 0===l&&(l=M.c[508].exports),l.Ks(e,t)},__wbindgen_object_clone_ref:function(e){return void 0===p&&(p=M.c[508].exports),p.m_(e)},__wbg_self_e7c1f827057f6584:function(){return void 0===d&&(d=M.c[508].exports),d.Q$()},__wbg_window_a09ec664e14b1b81:function(){return void 0===b&&(b=M.c[508].exports),b.ON()},__wbg_globalThis_87cbb8506fecf3a9:function(){return void 0===_&&(_=M.c[508].exports),_.I0()},__wbg_global_c85a9259e621f3db:function(){return void 0===g&&(g=M.c[508].exports),g.Gq()},__wbindgen_is_undefined:function(e){return void 0===h&&(h=M.c[508].exports),h.XP(e)},__wbg_new_15d3966e9981a196:function(e,t){return void 0===w&&(w=M.c[508].exports),w.Zj(e,t)},__wbg_resolve_fd40f858d9db1a04:function(e){return void 0===v&&(v=M.c[508].exports),v.vr(e)},__wbg_then_ec5db6d509eb475f:function(e,t){return void 0===m&&(m=M.c[508].exports),m.TO(e,t)},__wbg_then_f753623316e2873a:function(e,t,n){return void 0===y&&(y=M.c[508].exports),y.Zd(e,t,n)},__wbindgen_debug_string:function(e,t){return void 0===x&&(x=M.c[508].exports),x.fY(e,t)},__wbindgen_throw:function(e,t){return void 0===E&&(E=M.c[508].exports),E.Or(e,t)},__wbindgen_rethrow:function(e){return void 0===S&&(S=M.c[508].exports),S.nD(e)},__wbindgen_closure_wrapper102:function(e,t,n){return void 0===k&&(k=M.c[508].exports),k.J6(e,t,n)}}}}},P={235:[373]},M.w={},M.f.wasm=function(e,t){(P[e]||[]).forEach((function(n,r){var o=O[n];if(o)t.push(o);else{var i,a=j[n](),s=fetch(M.p+""+{235:{373:"4a5794d8f0a92804ee46"}}[e][n]+".module.wasm");i=a&&"function"==typeof a.then&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(s),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(s,a):s.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)})),t.push(O[n]=i.then((function(e){return M.w[n]=(e.instance||e).exports})))}}))},M(770)})();