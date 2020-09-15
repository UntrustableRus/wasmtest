const { main } = wasm_bindgen;

wasm_bindgen('./main_bg.wasm').then(function(){
main();
}).catch(console.error);
