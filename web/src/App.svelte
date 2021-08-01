<style global lang="postcss">
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>

<script>
      import SvelteTable from "svelte-table";

      export let processing;
      export let max_epoch;
      export let block_producer;
      export let foundation_addresses;
      export let send_addresses;

      // Use ES module import syntax to import functionality from the module
      // that we have compiled.
      //
      // Note that the `default` import is an initialization function which
      // will "boot" the module and make it ready to use. Currently browsers
      // don't support natively imported WebAssembly as an ES module, but
      // eventually the manual initialization won't be required!
      import init, { verify_enough_payments } from '../pkg/mina_fdv_rs.js';

      async function run() {
        processing = true;
        // First up we need to actually load the wasm file, so we use the
        // default export to inform it where the wasm file is located on the
        // server, and then we wait on the returned promise to wait for the
        // wasm to be loaded.
        //
        // It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
        // but there is also a handy default inside `init` function, which uses
        // `import.meta` to locate the wasm file relatively to js file.
        //
        // Note that instead of a string you can also pass in any of the
        // following things:
        //
        // * `WebAssembly.Module`
        //
        // * `ArrayBuffer`
        //
        // * `Response`
        //
        // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
        //
        // This gives you complete control over how the module is loaded
        // and compiled.
        //
        // Also note that the promise, when resolved, yields the wasm module's
        // exports which is the same as importing the `*_bg` module in other
        // modes
        await init();

        // And afterwards we can use all the functionality defined in wasm.
        const result = await verify_enough_payments(
            max_epoch,
            block_producer,
            foundation_addresses.split('\n'),
            send_addresses.split('\n'),
        );
        processing = false;
        console.log(`${JSON.stringify(result)}`);

        var rows = result;
        var columns = [
          {
            key: "address",
            title: "Address",
            value: v => v.address,
            sortable: true,
            headerClass: "text-left"
          },
          {
            key: "expected",
            title: "Expected",
            value: v => v.expected,
            sortable: true,
            headerClass: "text-left"
          },
          {
            key: "performed",
            title: "Performed",
            value: v => v.performed,
            sortable: true,
            headerClass: "text-left"
          },
          {
            key: "diff",
            title: "Diff",
            value: v => v.diff,
            sortable: true,
            headerClass: "text-left"
          },
        ];
        new SvelteTable({
          target: document.querySelector("#results_table"),
          props: { rows, columns }
        });

      }
    </script>


<div class="container mx-auto">
	<h1 class="text-lg font-bold">Mina Foundation Delegation Verifier</h1>
  <label for="max_epoch">Max Epoch</label>
  <input class="w-full" id="max_epoch" bind:value={max_epoch}>
  <label for="block_producer">Block Producer</label>
  <input class="w-full" id="block_producer" bind:value={block_producer}>
  <label for="foundation_addresses">Foundation Addresses</label>
  <textarea class="w-full" id="foundation_addresses" bind:value={foundation_addresses}></textarea>
  <label for="send_addresses">Send Addresses</label>
  <textarea class="w-full" id="send_addresses" bind:value={send_addresses}></textarea>
  <button class="w-auto bg-blue-500 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded" on:click={() => run().catch(() => {
    processing = false;
  })}>
    {#if !processing}
      Verify
    {/if}
    {#if processing}
      Processing...
    {/if}
  </button>
  <div id="results_table"></div>
</div>

<svelte:head>
  <base href="mina-fdv-rs">
  <title>Mina Foundation Delegation Verifier</title>
</svelte:head>


