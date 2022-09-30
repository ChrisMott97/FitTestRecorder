<script lang="ts">
	import BigTable from '../BigTable.svelte';
  import Database from 'tauri-plugin-sql-api';
  import type { Heading } from '../types'
  import { onMount } from 'svelte';
  import { open } from '@tauri-apps/api/dialog';
  import { fs } from '@tauri-apps/api';
  import { documentDir } from '@tauri-apps/api/path';

  const headings: Heading[] = [
    {key: 'name', label: 'Name'},
    {key: 'company', label: 'Company'},
    {key: 'location', label: 'Location'},
    {key: 'testDate', label: 'Date Tested'}
  ]

  type FitTestRecord = {
    name: string,
    company: string,
    location: string,
    testDate: string
  }

  let data: FitTestRecord[] = [];
  let newData: FitTestRecord[] = [];

  type FitTestField = {
    firstName: string,
    lastName: string,
    company: string,
    location: string,
    testDate: string
  }

	onMount(async () => {
    console.log("mounting");
    
    const db = await Database.load('sqlite:new_database.db');
    const res: FitTestField[] = await db.select('SELECT firstName, lastName, company, location, testDate from fitTestRecord');
    
    for (const person of res) {
      newData.push({
        name: `${person.firstName} ${person.lastName}`,
        company: person.company,
        location: person.location,
        testDate: new Date(person.testDate).toLocaleString()
      })
    };
    data = newData;

  })



	// onMount(async () => {
	// 	console.log('from the ts file!');
	// 	// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
	// 	Database.load('sqlite:new_database.db').then((db) => {
	// 		db.select('SELECT firstName, lastName, company, location, testDate from fitTestRecord').then(
	// 			(res: unknown) => {
	// 				for (const person of res) {
	// 					new_data.push([
	// 						`${person.firstName} ${person.lastName}`,
	// 						person.company,
	// 						person.location,
	// 						person.testDate
	// 					]);
	// 				}
	// 				data = new_data;
	// 			}
	// 		);
	// 	});
	// });

</script>

<p class="text-gray-500 text-center">Please select a person.</p>
<BigTable {headings} {data} />
