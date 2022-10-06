<script lang="ts">
	import type { Heading } from './types';
	export let headings: Heading[];
	export let data: Array<any & { visible: any }>;
	export let url: string = '';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function select(idx: number) {
		dispatch('message', {
			number: idx
		});
	}
</script>

<div class="flex flex-col md:mx-10 my-10">
	<div class="-my-2 overflow-x-auto sm:-mx-6">
		<div class="py-2 align-middle inline-block min-w-full sm:px-6">
			<div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
				<table class="min-w-full divide-y divide-gray-200">
					<thead class="bg-gray-50">
						<tr>
							{#each headings as { label }}
								<th
									scope="col"
									class="px-2 md:px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
									>{label}</th
								>
							{/each}
							<th scope="col" class="relative px-6 py-3">
								<span class="sr-only">Select</span>
							</th>
						</tr>
					</thead>
					<tbody>
						<!-- Odd row -->
						{#each data as item, i}
							<tr class:bg-green-300={item.valid} >
								{#each headings as { key }, j}
									<td
										class={j === 0
											? 'px-2 md:px-6 py-4 text-sm font-medium text-gray-900'
											: 'px-2 md:px-6 py-4 text-sm'}
									>
									{#if Array.isArray(item.visible[key])}
										{item.visible[key].join(', ')}
									{:else}
										{item.visible[key]}
									{/if}
									</td>
								{/each}
								<td class="px-2 md:px-6 py-4 text-right text-sm font-medium">
									<a
										href={url}
										class="text-indigo-600 hover:text-indigo-900"
										on:click={() => {
											select(i);
										}}>Select</a
									>
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
