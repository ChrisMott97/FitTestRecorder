<script>
  import BigTable from "./BigTable.svelte";
  let headings = ["Name", "Companies", "Last Tested"]
  let data = [
    [
      "Hanson Whatley 15-03-22",
      ["Hanson", "Robertet UK Ltd"],
      "2022-04-21 10:40:30"
],
    [
      "Hanson Whatley 20-03-22",
      ["Robertet UK Ltd"],
      "2022-04-20 10:40:30"
]
  ]
</script>

<p class="text-gray-500 text-center">Please select a database.</p>
<BigTable {headings} {data}></BigTable>