import {
    start_app,
} from '../.wasm/cql_admin';

(() => {
    const app = start_app();
    document.getElementById("app").innerHTML = app;
})();
