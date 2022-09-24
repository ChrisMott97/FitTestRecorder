<div class="mt-5 md:px-20">
  <div class="md:grid md:grid-cols-3 md:gap-6">
    <div class="mt-5 md:col-span-3 md:mt-0">
      <form action="#" method="POST">
        <div class="overflow-hidden shadow sm:rounded-md">
          <div class="bg-white px-4 py-5 sm:p-6">
            <div class="grid grid-cols-7 gap-6">

              <div class="col-span-3">
                <label for="first-name" class="block text-sm font-medium text-gray-700">First name</label>
                <input type="text" name="first-name" id="first-name" autocomplete="given-name" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-3">
                <label for="last-name" class="block text-sm font-medium text-gray-700">Last name</label>
                <input type="text" name="last-name" id="last-name" autocomplete="family-name" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-1 flex flex-col justify-end items-center">
                <button type="button" class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  Confirm
                </button>
              </div>

              <div class="col-span-3">
                <label for="company" class="block text-sm font-medium text-gray-700">Company</label>
                <input type="text" name="company" id="company" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-3">
                <label for="location" class="block text-sm font-medium text-gray-700">Location</label>
                <input type="text" name="location" id="location" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-1 flex flex-col justify-end items-center">
                <button type="button" class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  Confirm
                </button>
              </div>

              <div class="col-span-3">
                <label for="mask" class="block text-sm font-medium text-gray-700">Mask</label>
                <select id="mask" name="mask" class="mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm">
                  <option>Alpha Solway 2530V FFP3 [100]</option>
                  <option>3M 7502 Half Mask [100]</option>
                </select>
              </div>
              <div class="col-span-3">
                <label for="mask-size" class="block text-sm font-medium text-gray-700">Mask Size</label>
                <select id="mask-size" name="mask-size" class="mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm">
                  <option>OneSize</option>
                  <option>Small</option>
                  <option>Medium</option>
                  <option>Large</option>
                </select>
              </div>
              <div class="col-span-1 flex flex-col justify-end items-center">
                <button type="button" class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  Confirm
                </button>
              </div>

              
              <div class="col-span-6">
                <label for="signature" class="block text-sm font-medium text-gray-700">Signature</label>
                <canvas id="sig-canvas" width="620" height="160">
                  Get a better browser, bro.
                </canvas>
              </div>
              
            </div>
            <p class="text-gray-500 text-center">Please pass the laptop back to the operator.</p>
          </div>
          <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
            <button type="submit" class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Save</button>
          </div>
        </div>
      </form>
    </div>
  </div>
</div>
