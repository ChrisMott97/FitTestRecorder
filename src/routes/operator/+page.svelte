<script type="text/typescript">
  import SmallTable from "../SmallTable.svelte";

  const headings = ["Exercise", "Factor"]
  const data = [
    ["Normal Breathing", 7975],
    ["Deep Breathing", 1028],
    ["Head Side to Side", 2851],
    ["Head Up and Down", 6775],
    ["Talk Out Loud", 2957],
    ["Bending Over", 5138],
    ["Normal Breathing", 2670],
    ["Final Fit Factor", 2749]
  ]
</script>
<div class="flex flex-row gap-4 pt-5">
  <div>
    <SmallTable {headings} {data}></SmallTable>
  </div>
  <div>
    <div class="">
      <div class="md:grid md:grid-cols-3 md:gap-6">
        <div class="mt-5 md:col-span-3 md:mt-0">
          <form action="#" method="POST">
            <div class="overflow-hidden shadow sm:rounded-md">
              <div class="bg-white px-4 py-5 sm:p-6">
                <div class="grid grid-cols-7 gap-6">
                  <div class="col-span-7">
                    <label for="comment" class="block text-sm font-medium text-gray-700">Notes</label>
                    <div class="mt-1">
                      <textarea rows="4" name="comment" id="comment" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"></textarea>
                    </div>
                  </div>
                  <div class="col-span-6">
                    <label for="signature" class="block text-sm font-medium text-gray-700">Signature</label>
                    <canvas id="sig-canvas" width="500" height="160">
                      Get a better browser, bro.
                    </canvas>
                  </div>
                  
                </div>
              </div>
              <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
                <button type="submit" class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Save</button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
    
  </div>
</div>