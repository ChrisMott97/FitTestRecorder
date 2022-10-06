<script lang="ts">
  import type { Heading } from './types';
	export let headings: Heading[];
	export let data: Array<any & {visible: any}>;

  


</script>

<div class="flex flex-col">
  <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
    <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
      <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              {#each headings as {label}}
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{label}</th>
              {/each}
            </tr>
          </thead>
          <tbody>
            <!-- Odd row -->
            {#each data as item, i}
            <tr class="capitalize last:odd:border-double last:even:border-double last:odd:border-t-4 last:even:border-t-4 even:bg-gray-50 odd:bg-white">
              {#each headings as {key}, j}
                <td class="px-6 py-4 whitespace-nowrap text-sm first:font-medium text-gray-900" class:text-red-500="{item.visible[key] > 100000}">{item.visible[key]}</td>
              {/each}
            </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>