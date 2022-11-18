<script lang="ts">
	import { onMount } from 'svelte';
	import { open, message } from '@tauri-apps/api/dialog';
	import { fs, path } from '@tauri-apps/api';
	import { documentDir, publicDir, sep } from '@tauri-apps/api/path';
	import { testState } from './stores';
	// import Database from 'tauri-plugin-sql-api';
	import SQLite from 'tauri-plugin-sqlite-api';
	import type { Heading } from './types';
	import { goto } from '$app/navigation';
	import { toTitleCase } from './utilities';
	import { open as openInExplorer } from '@tauri-apps/api/shell';
	import { trace, info, error } from 'tauri-plugin-log-api';

	import BigTable from './BigTable.svelte';
	import Breadcrumb from './Breadcrumb.svelte';
	import { json } from '@sveltejs/kit';

	type SuggestedPerson = {
		id: number;
		url: string;
		valid: boolean;
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
		valid: false,
		visible: {
			database: '',
			name: '',
			company: '',
			location: '',
			testDate: ''
		}
	};


	async function gatherData() {
		info('landing: gathering data');
		databases = [];
		data = [];
		newData = [];
		potentialDatabases = [];

		//TODO: check children one level deep 
		const dirFiles = await fs.readDir(postPublicPath, { dir: fs.BaseDirectory.Public, recursive: true });
		for (const file of dirFiles) {
			info('checking '+file.path);
			if (file.children){
				for (const childFile of file.children){
					if (childFile.children){
						for (const subChildFile of childFile.children){
							if (subChildFile.path.split('.').at(-1) === 'db' && !subChildFile.children) {
								info('database found');
								potentialDatabases.push(subChildFile.path);
							}
						}
					}else{
						if (childFile.path.split('.').at(-1) === 'db') {
							info('database found');
							potentialDatabases.push(childFile.path);
						}
					}
				}
			}else{
				if (file.path.split('.').at(-1) === 'db') {
					info('database found');
					potentialDatabases.push(file.path);
				}
			}
		}
		if(potentialDatabases.length > 0){
			info(potentialDatabases.toString());
		}else{
			error("no potential databases found")
		}
		//TODO: show only unverified records as suggested - give option?
		for (const potentialDatabase of potentialDatabases) {
			const db = await SQLite.open(potentialDatabase);

			try {
				info('reading: ' + potentialDatabase);
				const version: Array<{ dbVersion: number }> = await db.select(
					'select dbVersion from dbInfo limit 1'
				);
				// update to use number from a config file for supported version
				if (version[0].dbVersion != 4) throw Error('wrong database version');
			} catch (e: Error | any) {
				error(e.toString());
				continue;
			}

			const latestTestArray: Array<FitTestField> = await db.select(
				'SELECT id, firstName, lastName, company, location, testDate, description from fitTestRecord where overallPass = 1 order by testDate desc limit 1'
			);
			if (latestTestArray.length == 0){
				error('no latest test found in ' + potentialDatabase);
				continue;
			}

			const companiesField: Array<{ company: string }> = await db.select(
				'select distinct company from fitTestRecord'
			);
			if (companiesField.length == 0){
				error('no companies found in ' + potentialDatabase);
				continue;
			}

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

			const companies = companiesField.map((company) =>
				toTitleCase(Object.values(company).at(0) as string)
			) as string[];

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
			valid: newestTestPerson.description.includes('verified'),
			visible: {
				name: `${newestTestPerson.firstName} ${newestTestPerson.lastName}`,
				database: newestDbFile.split(sep).at(-1) as string,
				company: newestTestPerson.company,
				location: newestTestPerson.location,
				testDate: new Date(newestTestDate).toLocaleString()
			}
		};

		data = newData;
	}

	onMount(gatherData);

	function handleSelect(e: CustomEvent) {
		const idx = e.detail.number;
		testState.update((n) => {
			n.database = databases[idx];
			info('selected database: '+n.database);
			info('state update: ' + JSON.stringify(n));
			return n;
		});
	}

	function handleSelected() {
		testState.update((n) => {
			n.database = suggestedPerson.url;
			n.person.id = suggestedPerson.id;
			info('selected person: '+suggestedPerson);
			info('selected database: '+n.database);
			info('state updated: ' + JSON.stringify(n));
			return n;
		});
	}

	async function openFolder(event: Event) {
		// opens a file using the default program:
		if (event.shiftKey) {
			info('opening logs folder');
			await openInExplorer(await path.logDir());
		} else {
			info('opening database folder');
			const url = (await path.publicDir()) + postPublicPath;
			const fitproExists = await fs.exists(postPublicPath, { dir: fs.BaseDirectory.Public })
			if (!fitproExists) {
				info("directory doesn't exist for databases; creating");
				fs.createDir(postPublicPath, { dir: fs.BaseDirectory.Public, recursive: true });
			}
			openInExplorer(url);
		}
	}

	async function findDatabase(){
		const selected = await open({
		multiple: false,
		filters: [{
			name: 'Database',
			extensions: ['db']
		}]
		});

		if (selected != null && typeof(selected) == "string") {
			const db = await SQLite.open(selected);
			try {
				info('reading from browse dialog: ' + selected);
				const version: Array<{ dbVersion: number }> = await db.select(
					'select dbVersion from dbInfo limit 1'
				);
				// update to use number from a config file for supported version
				if (version[0].dbVersion != 4) throw Error('wrong database version');

				testState.update((n) => {
					n.database = selected;
					info('selected database: '+n.database);
					info('state update: ' + JSON.stringify(n));
					return n;
				});
				goto('/people')
			} catch (e: Error | any) {
				error(e.toString());
				await message('Database is not valid or is wrong version', { title: 'Database Check', type: 'error' });
			}
		}
	}
</script>

<Breadcrumb active={1} />
<div class="py-4">
	<div class="text-left ml-20">
		<button on:click={gatherData} class="border py-2 px-6 rounded" type="button">Refresh</button>
		<button on:click={openFolder} class="border py-2 px-6 rounded" type="button"
			>Open Database and Signatures Folder</button
		>
	</div>
	{#if suggestedPerson.visible.database}
		<p class="text-gray-500 ml-20 pt-2 text-xl">
			Option 1: Select the suggested test to verify
		</p>
		<p class="text-gray-500 ml-20 pb-2 text-m">
			Green highlighted means it is verified already
		</p>
	{/if}
</div>
{#if suggestedPerson.visible.database}
	<BigTable
		headings={suggestedHeadings}
		data={[suggestedPerson]}
		url={suggestedUrl}
		on:message={handleSelected}
	/>
	<p class="text-gray-500 ml-20 py-8 text-xl">
		Option 2: Select a database file
	</p>
{/if}
{#if data.length > 0}
	<BigTable {headings} {data} {url} on:message={handleSelect} />
{:else}
	<p class="text-gray-500 text-center py-8 text-xl">
		No databases found, please ensure database files are in the folder above.
	</p>
{/if}

<div class="text-left ml-20 py-8">
	<button on:click={findDatabase} class="border py-2 px-6 rounded" type="button">Browse for database file</button>
</div>