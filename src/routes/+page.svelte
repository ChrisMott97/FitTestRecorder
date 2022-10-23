<script lang="ts">
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { fs, path } from '@tauri-apps/api';
	import { documentDir, publicDir, sep } from '@tauri-apps/api/path';
	import { testState } from './stores';
	import Database from 'tauri-plugin-sql-api';
	import type { Heading } from './types';
	import { goto } from '$app/navigation';
	import { toTitleCase } from './utilities';
	import { open as openInExplorer } from '@tauri-apps/api/shell';

	import BigTable from './BigTable.svelte';
	import Breadcrumb from './Breadcrumb.svelte';

	type SuggestedPerson = {
		id: number;
		url: string;
		visible: {
			database: string;
			name: string;
			company: string;
			location: string;
			testDate: string;
		};
	};

	type FitTestField = {
		id: number;
		firstName: string;
		lastName: string;
		company: string;
		location: string;
		testDate: string;
		description: string;
	};

	type DBFile = {
		visible: {
			name: string;
			companies: string[];
			testDate: string;
		};
	};

	const url = '/people';
	const suggestedUrl = '/person';
	const postPublicPath = `Documents${sep}TSI${sep}fitpro`;

	const headings: Heading[] = [
		{ key: 'name', label: 'Name' },
		{ key: 'companies', label: 'Companies' },
		{ key: 'testDate', label: 'Last Tested' }
	];

	const suggestedHeadings: Heading[] = [
		{ key: 'database', label: 'Database' },
		{ key: 'name', label: 'Name' },
		{ key: 'company', label: 'Company' },
		{ key: 'location', label: 'Location' },
		{ key: 'testDate', label: 'Date Tested' }
	];

	let data: DBFile[] = [];
	let newData: DBFile[] = [];

	let potentialDatabases: string[] = [];
	let databases: string[] = [];

	let newestTestDate: string = '';
	let newestDbFile: string;
	let newestTestPerson: FitTestField;

	let suggestedPerson: SuggestedPerson = {
		id: 0,
		url: '',
		visible: {
			database: '',
			name: '',
			company: '',
			location: '',
			testDate: ''
		}
	};

	async function gatherData(){
		databases = [];
		data = [];
		newData = [];
		potentialDatabases = [];

		//TODO: change database adapter to be able to close connection
		const dirFiles = await fs.readDir(postPublicPath, { dir: fs.BaseDirectory.Public });

		for (const file of dirFiles) {
			if (file.path.split('.').at(-1) === 'db') {
				potentialDatabases.push(file.path);
			}
		}
		//TODO: show only unverified records as suggested
		for (const potentialDatabase of potentialDatabases) {
			let db = Database.get('sqlite:' + potentialDatabase);
			try {
				await db.select("select 1");
			} catch (error) {
				db = await Database.load('sqlite:' + potentialDatabase)
			}
			//TODO: handle not an sqlite file

			const latestTestArray: Array<FitTestField> = await db.select(
				'SELECT id, firstName, lastName, company, location, testDate, description from fitTestRecord where overallPass = 1 order by testDate desc limit 1'
			);
			if (latestTestArray.length == 0) continue;

			const companiesField: Array<{ company: string }> = await db.select(
				'select distinct company from fitTestRecord'
			);
			if (companiesField.length == 0) continue;

			const latestTest: FitTestField = latestTestArray[0];
			latestTest.firstName = toTitleCase(latestTest.firstName);
			latestTest.lastName = toTitleCase(latestTest.lastName);
			latestTest.company = toTitleCase(latestTest.company);
			latestTest.location = toTitleCase(latestTest.location);


			if (!newestTestDate || new Date(latestTest.testDate) > new Date(newestTestDate)) {
				newestDbFile = potentialDatabase;
				newestTestDate = latestTest.testDate;
				newestTestPerson = latestTest;
			}

			const companies = companiesField.map((company) => toTitleCase(Object.values(company).at(0) as string)) as string[];

			const dbFilename = potentialDatabase.split(sep).at(-1);
			if (typeof dbFilename === 'string') {
				newData.push({
					visible: {
						name: dbFilename,
						companies: companies,
						testDate: new Date(latestTest.testDate).toLocaleString()
					}
				});
			}

			databases.push(potentialDatabase);
		}
		suggestedPerson = {
			id: newestTestPerson.id,
			url: newestDbFile,
			visible: {
				name: `${newestTestPerson.firstName} ${newestTestPerson.lastName}`,
				database: newestDbFile.split(sep).at(-1) as string,
				company: newestTestPerson.company,
				location: newestTestPerson.location,
				testDate: new Date(newestTestDate).toLocaleString()
			}
		}

		data = newData;
	}

	onMount(gatherData);

	function handleSelect(e: CustomEvent) {
		const idx = e.detail.number;
		testState.update((n) => {
			n.database = databases[idx];
			return n;
		});
	}

	function handleSelected() {
		testState.update((n)=>{
			n.database = suggestedPerson.url;
			n.person.id = suggestedPerson.id;
			return n;
		})
	}

	async function openFolder(){
		// opens a file using the default program:
		const url = await path.publicDir() + postPublicPath
		console.log(url)
		await openInExplorer(url).then(console.log).catch(console.log)
	}
</script>
<Breadcrumb active={1}/>
<div class="py-4">
	<p class="text-gray-500 text-center py-4 text-xl">The suggested test below is the last test done - if this isn't up to date, click the Refresh button.</p>
	<div class="text-left ml-20">
		<button on:click={gatherData} class="border py-2 px-6 rounded" type="button">Refresh</button>
		<button on:click={openFolder} class="border py-2 px-6 rounded" type="button">Open Database Folder</button>
	</div>
</div>
<BigTable headings={suggestedHeadings} data={[suggestedPerson]} url={suggestedUrl} on:message={handleSelected} />
<p class="text-gray-500 text-center py-8 text-xl">- OR - <br/> Select a database file manually</p>
<BigTable {headings} {data} {url} on:message={handleSelect} />
