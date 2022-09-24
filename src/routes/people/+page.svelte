<script>
  import BigTable from "../BigTable.svelte";
  const headings = ["Name", "Company", "Location", "Date Tested"]
  const data = [
    ["Chris Mott", "Howbout", "London", "2022-09-24 17:00:00"]
  ]
</script>

<p class="text-gray-500 text-center">Please select a person.</p>
<BigTable {headings} {data}></BigTable>