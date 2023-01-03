import init, { Blackjack } from "../../../blackjack-wasm/pkg/blackjack_wasm.js"
import wasmBinary from '../../../blackjack-wasm/pkg/blackjack_wasm_bg.wasm'

let initPromise: ReturnType<typeof init>;
if (typeof document !== "undefined") {
  initPromise = init(wasmBinary);
}

export default function Index() {

 
  return (
    <>
    
    </>
  );
}
