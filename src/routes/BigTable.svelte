<script type="text/typescript">
  export let headings: String[];
  export let data: Array<any>;
</script>

<div class="flex flex-col m-10">
  <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
    <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
      <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              {#each headings as heading}
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{heading}</th>
              {/each}
              <th scope="col" class="relative px-6 py-3">
                <span class="sr-only">Edit</span>
              </th>
            </tr>
          </thead>
          <tbody>
            <!-- Odd row -->
            {#each data as item, i}
            <tr class={i % 2 !== 0 ? "bg-gray-50": "bg-white"}>
              {#each item as prop, j}
                <td class={j === 0 ? "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900":"px-6 py-4 whitespace-nowrap text-sm"}>{prop}</td>
              {/each}
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <a href="#" class="text-indigo-600 hover:text-indigo-900">Select</a>
              </td>
            </tr>
            {/each}
            <!-- <tr class="bg-white">
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">Hanson Whatley 15-03-22</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">Hanson, Robertet UK Ltd</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">2022-04-21 10:40:30</td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <a href="#" class="text-indigo-600 hover:text-indigo-900">Select</a>
              </td>
            </tr>

            <tr class="bg-gray-50">
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">...</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">...</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">...</td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <a href="#" class="text-indigo-600 hover:text-indigo-900">Select</a>
              </td>
            </tr> -->

            <!-- More people... -->
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>