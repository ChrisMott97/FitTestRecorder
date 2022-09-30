<script lang="ts">
	import BigTable from '../BigTable.svelte';
  import Database from 'tauri-plugin-sql-api';
  import { onMount } from 'svelte';
  import { open } from '@tauri-apps/api/dialog';
  import { fs } from '@tauri-apps/api';
  import { documentDir } from '@tauri-apps/api/path';

	const headings = ['Name', 'Company', 'Location', 'Date Tested'];
	let data: String[][] = [['Chris Mott', 'Howbout', 'London', '2022-09-24 17:00:00']];
  let new_data: String[][] = [];

  type fitTestRecord = {
    firstName: string,
    lastName: string,
    company: string,
    location: string,
    testDate: string
  }

	onMount(async () => {
    console.log("mounting");
    
    const db = await Database.load('sqlite:');
    const res: fitTestRecord[] = await db.select('SELECT firstName, lastName, company, location, testDate from fitTestRecord');
    
    for (const person of res) {
      new_data.push([
        `${person.firstName} ${person.lastName}`,
        person.company,
        person.location,
        person.testDate
      ])
    };
    data = new_data;

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
<BigTable {headings} bind:data />
