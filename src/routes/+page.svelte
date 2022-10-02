<script lang="ts">
	import { onMount } from 'svelte';
  import { open } from '@tauri-apps/api/dialog';
  import { fs } from '@tauri-apps/api';
  import { documentDir } from '@tauri-apps/api/path';
  import { testState } from './stores'
  import Database from 'tauri-plugin-sql-api';
  import type { Heading } from './types';
  import { goto } from '$app/navigation';

	import BigTable from './BigTable.svelte';
  
  const url = "/people";

  let selected = "";

  const headings: Heading[] = [
    {key: 'name', label: 'Name'},
    {key: 'companies', label: 'Companies'},
    {key: 'testDate', label: 'Last Tested'}
  ]

  type DBFile = {
    visible: {
      name: string,
      companies: string[],
      testDate: string
    }
  }

  let data: DBFile[] = []

  let newData: DBFile[] = []

  let dbs:string[] = []

  onMount(async ()=>{

    const selected = await open({
      directory: true,
      defaultPath: await documentDir()
    });

    if(typeof selected === 'string'){
      const dirCon = await fs.readDir(selected);
      for (const file of dirCon) {
        if(file.path.split('.').at(-1) === 'db'){
          dbs.push(file.path)
        }
      }
    }

    for (const dbFile of dbs) {
      const db = await Database.load('sqlite:' + dbFile);
      const testDateField: Array<{testDate: string}> = await db.select('SELECT testDate from fitTestRecord ORDER BY testDate DESC LIMIT 1');
      const companiesField: Array<{company: string}> = await db.select('select distinct company from fitTestRecord');
      // TODO: replace with map
      let companies = [];
      for (const company of companiesField) {
        companies.push(Object.values(company)[0])
      }

      const dbFilename = dbFile.split('/').at(-1);
      if(typeof dbFilename === 'string'){
        newData.push({
          visible: {
            name: dbFilename, 
            companies: companies, 
            testDate: new Date(testDateField[0].testDate).toLocaleString()
          }
        })
      }

    }
    data = newData
    
  })
  
  function handleSelect(e: CustomEvent){
    const idx = e.detail.number;
    testState.update((n)=>{
      n.database = dbs[idx]
      return n
    })
  }

</script>

<p class="text-gray-500 text-center">Please select a database.</p>
{selected}
<BigTable {headings} {data} {url} on:message={handleSelect}/>
