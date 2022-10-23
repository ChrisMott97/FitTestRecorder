<script lang="ts">
  export let active = 1;
</script>

<!-- This example requires Tailwind CSS v2.0+ -->
<nav class="flex justify-center" aria-label="Breadcrumb">
  <ol class="flex items-center space-x-4">

    <li>
      <div>
				<p class="ml-4 text-sm font-medium" class:text-gray-900={active >= 1} class:text-gray-400={active < 1}>Home</p>
      </div>
    </li>

    <li>
      <div class="flex items-center">
        <!-- Heroicon name: solid/chevron-right -->
        <svg class="flex-shrink-0 h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
        <p class="ml-4 text-sm font-medium" class:text-gray-900={active >= 2} class:text-gray-400={active < 2}>People</p>
      </div>
    </li>

    <li>
      <div class="flex items-center">
        <!-- Heroicon name: solid/chevron-right -->
        <svg class="flex-shrink-0 h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
        <p class="ml-4 text-sm font-medium" class:text-gray-900={active >= 3} class:text-gray-400={active < 3} aria-current="page">Client Verify</p>
      </div>
    </li>

		<li>
      <div class="flex items-center">
        <!-- Heroicon name: solid/chevron-right -->
        <svg class="flex-shrink-0 h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
        </svg>
        <p class="ml-4 text-sm font-medium" class:text-gray-900={active >= 4} class:text-gray-400={active < 4} aria-current="page">Operator Verify</p>
      </div>
    </li>

  </ol>
</nav>