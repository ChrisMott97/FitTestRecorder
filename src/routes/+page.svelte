<script lang="ts">
	import { onMount } from 'svelte';
  import { open } from '@tauri-apps/api/dialog';
  import { fs } from '@tauri-apps/api';
  import { documentDir } from '@tauri-apps/api/path';
  import Database from 'tauri-plugin-sql-api';

	import BigTable from './BigTable.svelte';
	let headings = ['Name', 'Companies', 'Last Tested'];
  let data = [['new_database', ['Hanson'], '2022-09-29 15:56:00']]
  let new_data: string[][] = []


  let dbs:string[] = []

  type companyInfo = {
    firstName: string,
    lastName: string,
    company: string,
    location: string,
    testDate: string
  }

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
      const db = await Database.load('sqlite:'+dbFile);
      const testDateField: Array<{testDate: string}> = await db.select('SELECT testDate from fitTestRecord ORDER BY testDate DESC LIMIT 1');
      const companiesField: Array<{company: string}> = await db.select('select distinct company from fitTestRecord');
      let companies = [];
      for (const company of companiesField) {
        companies.push(Object.values(company)[0])
      }

      new_data.push([dbFile.split('/').at(-1), companies, new Date(testDateField[0].testDate).toLocaleString()])
    }
    data = new_data
    
  })

</script>

<p class="text-gray-500 text-center">Please select a database.</p>
<BigTable {headings} {data} />
