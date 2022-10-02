<script lang="ts">
	import BigTable from '../BigTable.svelte';
  import Database from 'tauri-plugin-sql-api';
  import type { Heading } from '../types'
  import { onMount } from 'svelte';
  import { testState } from '../stores'

  const headings: Heading[] = [
    {key: 'name', label: 'Name'},
    {key: 'company', label: 'Company'},
    {key: 'location', label: 'Location'},
    {key: 'testDate', label: 'Date Tested'}
  ]

  type FitTestRecord = {
    id: number,
    visible:{
      name: string,
      company: string,
      location: string,
      testDate: string
    }
  }

  let data: FitTestRecord[] = [];
  let newData: FitTestRecord[] = [];

  type FitTestField = {
    id: number,
    firstName: string,
    lastName: string,
    company: string,
    location: string,
    testDate: string
  }

  let db: Database;

  const url = '/person'

	onMount(async () => {

    testState.subscribe(async newTestState => {
      let databaseURL = newTestState.database;
      
      db = await Database.load('sqlite:' + databaseURL);
      const res: FitTestField[] = await db.select('SELECT id, firstName, lastName, company, location, testDate from fitTestRecord order by testDate desc');
      
      for (const person of res) {
        newData.push({
          id: person.id,
          visible: {
            name: `${person.firstName} ${person.lastName}`,
            company: person.company,
            location: person.location,
            testDate: new Date(person.testDate).toLocaleString()
          }
        })
      };
      data = newData;
    })

  })

  function handleSelect(e: CustomEvent){
    const idx = e.detail.number;
    const id = data[idx].id;

    testState.update((n)=>{
      n.person.id = id;
      return n
    })
  }

</script>

<p class="text-gray-500 text-center">Please select a person.</p>
<BigTable {headings} {data} {url} on:message={handleSelect}/>
