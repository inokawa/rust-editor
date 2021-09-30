(()=>{"use strict";var e,t,n,r,o,i,a,s,c,u,l,f,p,d,b,_,g,h,m,w,v,y,x={534:(e,t,n)=>{const r=Symbol("Comlink.proxy"),o=Symbol("Comlink.endpoint"),i=Symbol("Comlink.releaseProxy"),a=Symbol("Comlink.thrown"),s=e=>"object"==typeof e&&null!==e||"function"==typeof e,c=new Map([["proxy",{canHandle:e=>s(e)&&e[r],serialize(e){const{port1:t,port2:n}=new MessageChannel;return u(e,t),[n,[n]]},deserialize:e=>(e.start(),p(e,[],undefined))}],["throw",{canHandle:e=>s(e)&&a in e,serialize({value:e}){let t;return t=e instanceof Error?{isError:!0,value:{message:e.message,name:e.name,stack:e.stack}}:{isError:!1,value:e},[t,[]]},deserialize(e){if(e.isError)throw Object.assign(new Error(e.value.message),e.value);throw e.value}}]]);function u(e,t=self){t.addEventListener("message",(function n(o){if(!o||!o.data)return;const{id:i,type:s,path:c}=Object.assign({path:[]},o.data),f=(o.data.argumentList||[]).map(g);let p;try{const t=c.slice(0,-1).reduce(((e,t)=>e[t]),e),n=c.reduce(((e,t)=>e[t]),e);switch(s){case 0:p=n;break;case 1:t[c.slice(-1)[0]]=g(o.data.value),p=!0;break;case 2:p=n.apply(t,f);break;case 3:p=function(e){return Object.assign(e,{[r]:!0})}(new n(...f));break;case 4:{const{port1:t,port2:n}=new MessageChannel;u(e,n),p=function(e,t){return b.set(e,t),e}(t,[t])}break;case 5:p=void 0}}catch(e){p={value:e,[a]:0}}Promise.resolve(p).catch((e=>({value:e,[a]:0}))).then((e=>{const[r,o]=_(e);t.postMessage(Object.assign(Object.assign({},r),{id:i}),o),5===s&&(t.removeEventListener("message",n),l(t))}))})),t.start&&t.start()}function l(e){(function(e){return"MessagePort"===e.constructor.name})(e)&&e.close()}function f(e){if(e)throw new Error("Proxy has been released and is not useable")}function p(e,t=[],n=function(){}){let r=!1;const a=new Proxy(n,{get(n,o){if(f(r),o===i)return()=>h(e,{type:5,path:t.map((e=>e.toString()))}).then((()=>{l(e),r=!0}));if("then"===o){if(0===t.length)return{then:()=>a};const n=h(e,{type:0,path:t.map((e=>e.toString()))}).then(g);return n.then.bind(n)}return p(e,[...t,o])},set(n,o,i){f(r);const[a,s]=_(i);return h(e,{type:1,path:[...t,o].map((e=>e.toString())),value:a},s).then(g)},apply(n,i,a){f(r);const s=t[t.length-1];if(s===o)return h(e,{type:4}).then(g);if("bind"===s)return p(e,t.slice(0,-1));const[c,u]=d(a);return h(e,{type:2,path:t.map((e=>e.toString())),argumentList:c},u).then(g)},construct(n,o){f(r);const[i,a]=d(o);return h(e,{type:3,path:t.map((e=>e.toString())),argumentList:i},a).then(g)}});return a}function d(e){const t=e.map(_);return[t.map((e=>e[0])),(n=t.map((e=>e[1])),Array.prototype.concat.apply([],n))];var n}const b=new WeakMap;function _(e){for(const[t,n]of c)if(n.canHandle(e)){const[r,o]=n.serialize(e);return[{type:3,name:t,value:r},o]}return[{type:0,value:e},b.get(e)||[]]}function g(e){switch(e.type){case 3:return c.get(e.name).deserialize(e.value);case 0:return e.value}}function h(e,t,n){return new Promise((r=>{const o=new Array(4).fill(0).map((()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16))).join("-");e.addEventListener("message",(function t(n){n.data&&n.data.id&&n.data.id===o&&(e.removeEventListener("message",t),r(n.data))})),e.start&&e.start(),e.postMessage(Object.assign({id:o},t),n)}))}var m=function(e,t,n,r){return new(n||(n=Promise))((function(o,i){function a(e){try{c(r.next(e))}catch(e){i(e)}}function s(e){try{c(r.throw(e))}catch(e){i(e)}}function c(e){var t;e.done?o(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(a,s)}c((r=r.apply(e,t||[])).next())}))},w=function(e,t){var n,r,o,i,a={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:s(0),throw:s(1),return:s(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function s(i){return function(s){return function(i){if(n)throw new TypeError("Generator is already executing.");for(;a;)try{if(n=1,r&&(o=2&i[0]?r.return:i[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,i[1])).done)return o;switch(r=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return a.label++,{value:i[1],done:!1};case 5:a.label++,r=i[1],i=[0];continue;case 7:i=a.ops.pop(),a.trys.pop();continue;default:if(!((o=(o=a.trys).length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){a=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){a.label=i[1];break}if(6===i[0]&&a.label<o[1]){a.label=o[1],o=i;break}if(o&&a.label<o[2]){a.label=o[2],a.ops.push(i);break}o[2]&&a.ops.pop(),a.trys.pop();continue}i=t.call(e,a)}catch(e){i=[6,e],r=0}finally{n=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,s])}}},v=[];u({init:function(e){return m(void 0,void 0,void 0,(function(){return w(this,(function(t){switch(t.label){case 0:return self.term={write:e,get_key:function(){var e=v.shift();return null==e?void 0:e.key}},[4,n.e(235).then(n.bind(n,235))];case 1:return t.sent(),[2]}}))}))},send_key:function(e,t){return m(void 0,void 0,void 0,(function(){return w(this,(function(n){return v.push({key:e,ctrl:t}),[2]}))}))}})}},k={};function S(e){var t=k[e];if(void 0!==t)return t.exports;var n=k[e]={id:e,loaded:!1,exports:{}};return x[e](n,n.exports,S),n.loaded=!0,n.exports}S.m=x,S.c=k,S.d=(e,t)=>{for(var n in t)S.o(t,n)&&!S.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},S.f={},S.e=e=>Promise.all(Object.keys(S.f).reduce(((t,n)=>(S.f[n](e,t),t)),[])),S.u=e=>e+".js",S.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),S.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),S.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),S.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;S.g.importScripts&&(e=S.g.location+"");var t=S.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");n.length&&(e=n[n.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),S.p=e})(),(()=>{var e={872:1};S.f.i=(t,n)=>{e[t]||importScripts(S.p+S.u(t))};var t=self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[],n=t.push.bind(t);t.push=t=>{var[r,o,i]=t;for(var a in o)S.o(o,a)&&(S.m[a]=o[a]);for(i&&i(S);r.length;)e[r.pop()]=1;n(t)}})(),w={},v={716:function(){return{"./index_bg.js":{__wbg_write_daa581465730aee1:function(t,n){return void 0===e&&(e=S.c[838].exports),e.eV(t,n)},__wbg_getkey_17de46949815a52d:function(e){return void 0===t&&(t=S.c[838].exports),t.Xl(e)},__wbindgen_cb_drop:function(e){return void 0===n&&(n=S.c[838].exports),n.G6(e)},__wbindgen_object_drop_ref:function(e){return void 0===r&&(r=S.c[838].exports),r.ug(e)},__wbindgen_string_new:function(e,t){return void 0===o&&(o=S.c[838].exports),o.h4(e,t)},__wbg_setTimeout_119f2ec17c176110:function(e,t,n){return void 0===i&&(i=S.c[838].exports),i.kZ(e,t,n)},__wbg_now_44a034aa2e1d73dd:function(e){return void 0===a&&(a=S.c[838].exports),a.OK(e)},__wbg_newnoargs_be86524d73f67598:function(e,t){return void 0===s&&(s=S.c[838].exports),s.wg(e,t)},__wbg_get_4d0f21c2f823742e:function(e,t){return void 0===c&&(c=S.c[838].exports),c.XJ(e,t)},__wbg_call_888d259a5fefc347:function(e,t){return void 0===u&&(u=S.c[838].exports),u.BT(e,t)},__wbindgen_object_clone_ref:function(e){return void 0===l&&(l=S.c[838].exports),l.m_(e)},__wbg_self_c6fbdfc2918d5e58:function(){return void 0===f&&(f=S.c[838].exports),f.JX()},__wbg_window_baec038b5ab35c54:function(){return void 0===p&&(p=S.c[838].exports),p.xd()},__wbg_globalThis_3f735a5746d41fbd:function(){return void 0===d&&(d=S.c[838].exports),d.ud()},__wbg_global_1bc0b39582740e95:function(){return void 0===b&&(b=S.c[838].exports),b.PT()},__wbindgen_is_undefined:function(e){return void 0===_&&(_=S.c[838].exports),_.XP(e)},__wbindgen_debug_string:function(e,t){return void 0===g&&(g=S.c[838].exports),g.fY(e,t)},__wbindgen_throw:function(e,t){return void 0===h&&(h=S.c[838].exports),h.Or(e,t)},__wbindgen_closure_wrapper42:function(e,t,n){return void 0===m&&(m=S.c[838].exports),m.yr(e,t,n)}}}}},y={235:[716]},S.w={},S.f.wasm=function(e,t){(y[e]||[]).forEach((function(n,r){var o=w[n];if(o)t.push(o);else{var i,a=v[n](),s=fetch(S.p+""+{235:{716:"8897137393a21bbae537"}}[e][n]+".module.wasm");i=a instanceof Promise&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(s),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(s,a):s.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)})),t.push(w[n]=i.then((function(e){return S.w[n]=(e.instance||e).exports})))}}))},S(534)})();