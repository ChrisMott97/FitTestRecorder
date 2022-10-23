<script lang="ts">
	import BigTable from '../BigTable.svelte';
	import SQLite from 'tauri-plugin-sqlite-api';
	import type { Heading } from '../types';
	import { onMount } from 'svelte';
	import { testState } from '../stores';
	import Breadcrumb from '../Breadcrumb.svelte'
	import { toTitleCase } from "../utilities";
	import { trace, info, error } from 'tauri-plugin-log-api'


	const headings: Heading[] = [
		{ key: 'name', label: 'Name' },
		{ key: 'company', label: 'Company' },
		{ key: 'location', label: 'Location' },
		{ key: 'testDate', label: 'Date Tested' }
	];

	type FitTestRecord = {
		id: number;
		valid: boolean;
		visible: {
			name: string;
			company: string;
			location: string;
			testDate: string;
		};
	};

	let data: FitTestRecord[] = [];
	let newData: FitTestRecord[] = [];

	type FitTestField = {
		id: number;
		firstName: string;
		lastName: string;
		company: string;
		location: string;
		testDate: string;
		description: string;
	};

	let db: SQLite;

	const url = '/person';

	onMount(async () => {
		testState.subscribe(async (newTestState) => {
			let databaseURL = newTestState.database;

			db = await SQLite.open(databaseURL);
			const res: FitTestField[] = await db.select(
				'SELECT id, firstName, lastName, company, location, testDate, description from fitTestRecord where overallPass = 1 order by testDate desc'
			);

			for (const person of res) {
				newData.push({
					id: person.id,
					valid: person.description.includes('verified'),
					visible: {
						name: toTitleCase(`${person.firstName} ${person.lastName}`),
						company: toTitleCase(person.company),
						location: toTitleCase(person.location),
						testDate: new Date(person.testDate).toLocaleString()
					}
				});
			}
			info("People page: Gather people: "+JSON.stringify(newData))
			data = newData;
		});
	});

	function handleSelect(e: CustomEvent) {
		const idx = e.detail.number;
		const id = data[idx].id;

		testState.update((n) => {
			n.person.id = id;
			info("People page: State update: "+JSON.stringify(n))
			return n;
		});
	}
</script>
<Breadcrumb active={2}/>
<div class="py-4">
	<p class="text-gray-500 text-center pt-2 text-xl">Please select a person.</p>
	<p class="text-gray-500 text-center pb-2 text-m">Records highlighted green have been verified already.</p>
	<div class="text-left ml-20">
		<a href="/"><button class="border py-2 px-6 rounded">Back</button></a>
	</div>
</div>
<BigTable {headings} {data} {url} on:message={handleSelect} />
