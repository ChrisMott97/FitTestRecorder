<script lang="ts">
	import Database from 'tauri-plugin-sql-api';
	import { onMount } from 'svelte';
	import { testState } from '../stores';
	import { fs } from '@tauri-apps/api';
	import { documentDir, publicDir, sep } from '@tauri-apps/api/path';
	import SignaturePad from 'signature_pad';
	import Breadcrumb from '../Breadcrumb.svelte'
	import { toTitleCase } from '../utilities'
	import { updated } from '$app/stores';

	let linesDisabled = [false, false, false];
	const postPublicPath = `Documents${sep}TSI${sep}fitpro`;

	type FitTestPerson = {
		id: number;
		idNumber: string;
		firstName: string;
		lastName: string;
		company: string;
		location: string;
		maskDescription: string;
		maskSize: string;
	};

	type Mask = {
		maskManufacturer: string;
		maskModel: string;
		maskStyle: string;
		ffPassLevel: number;
		n95: number;
	};

	let fitTestPerson: FitTestPerson = {
		id: 0,
		idNumber: '',
		firstName: '',
		lastName: '',
		company: '',
		location: '',
		maskDescription: '',
		maskSize: ''
	};

	let updatedFitTestPerson: FitTestPerson = {
		id: 0,
		idNumber: '',
		firstName: '',
		lastName: '',
		company: '',
		location: '',
		maskDescription: '',
		maskSize: ''
	};

	let db: Database;
	let masksArr: string[] = [];

	let signaturePad: SignaturePad;
	let signed = false;

	function toggleLine(lineIndex: number) {
		linesDisabled[lineIndex] = !linesDisabled[lineIndex];
	}

	async function save() {
		const imgUrl = signaturePad.toDataURL();

		// const file = await fs.readDir(postPublicPath, { dir: fs.BaseDirectory.Public });
		const newDir = postPublicPath + sep + db.path.split(sep).at(-1)?.split('.').at(0);
		fs.exists(newDir, { dir: fs.BaseDirectory.Public }).then((exists) => {
			if (!exists) {
				fs.createDir(newDir, { dir: fs.BaseDirectory.Public });
			}
		});

		fetch(imgUrl)
			.then((res) => res.blob())
			.then((blob) => {
				blob.arrayBuffer().then((aB) => {
					fs.writeBinaryFile(
						newDir +
							sep +
							`${updatedFitTestPerson.firstName}-${
								updatedFitTestPerson.lastName
							}-${updatedFitTestPerson.idNumber.replaceAll('/', '-')}.png`,
						aB,
						{ dir: fs.BaseDirectory.Public }
					);
				});
			});

		const maskInfo: Mask[] = await db.select(
			'select maskManufacturer, maskModel, maskStyle, ffPassLevel, n95 from maskRecord where maskDescription = $1',
			[updatedFitTestPerson.maskDescription]
		);

		maskInfo[0].n95 = maskInfo[0].n95 | 0;
		const { maskManufacturer, maskModel, maskStyle, ffPassLevel, n95 } = maskInfo[0];

		// update mask details
		await db.execute(
			'UPDATE fitTestRecord SET maskSize = $1, maskManufacturer = $2, maskModel = $3, maskStyle = $4, maskDescription = $5, ffPassLevel = $6, n95 = $7 WHERE id = $8',
			[
				updatedFitTestPerson.maskSize,
				maskManufacturer,
				maskModel,
				maskStyle,
				updatedFitTestPerson.maskDescription,
				ffPassLevel,
				n95,
				fitTestPerson.id
			]
		);

		// update personal details on all records
		await db.execute(
			'UPDATE fitTestRecord SET firstName = $1, lastName = $2, company = $3, location = $4 WHERE firstName = $5 and lastName = $6 and company = $7 and location = $8',
			[
				updatedFitTestPerson.firstName,
				updatedFitTestPerson.lastName,
				updatedFitTestPerson.company,
				updatedFitTestPerson.location,
				fitTestPerson.firstName,
				fitTestPerson.lastName,
				fitTestPerson.company,
				fitTestPerson.location
			]
		);

		let idNumberVariant1 = fitTestPerson.idNumber;
		let idNumberVariant2 = fitTestPerson.idNumber;
		let idNumberVariant3 = fitTestPerson.idNumber;
		let idNumberSplit = fitTestPerson.idNumber.split('/');
		let idNumberSplitVariant1 = fitTestPerson.idNumber.split('/');
		let idNumberSplitVariant2 = fitTestPerson.idNumber.split('/');
		let idNumberSplitVariant3 = fitTestPerson.idNumber.split('/');

		if (idNumberSplit.length == 3) {
			if (idNumberSplit[2].length == 2) {
				idNumberSplitVariant1[2] = '19' + idNumberSplitVariant1[2];
				idNumberSplitVariant2[2] = '20' + idNumberSplitVariant2[2];
				idNumberVariant1 = idNumberSplitVariant1.join('/');
				idNumberVariant2 = idNumberSplitVariant2.join('/');
			} else if (idNumberSplit[2].length == 4) {
				idNumberSplitVariant3[2] = idNumberSplitVariant3[2].slice(2);
				idNumberVariant3 = idNumberSplitVariant3.join('/');
			}
		}

		const { rowsAffected } = await db.execute(
			'UPDATE peopleRecord SET firstName = $1, lastName = $2, company = $3, location = $4 WHERE firstName = $5 AND lastName = $6 AND company = $7 AND location = $8 AND (idNumber = $9 OR idNumber = $10 OR idNumber = $11 OR idNumber = $12)',
			[
				updatedFitTestPerson.firstName,
				updatedFitTestPerson.lastName,
				updatedFitTestPerson.company,
				updatedFitTestPerson.location,
				fitTestPerson.firstName,
				fitTestPerson.lastName,
				fitTestPerson.company,
				fitTestPerson.location,
				fitTestPerson.idNumber,
				idNumberVariant1,
				idNumberVariant2,
				idNumberVariant3
			]
		);

		if (rowsAffected == 0) {
			const { rowsAffected: success } = await db.execute(
				'UPDATE peopleRecord SET firstName = $1, lastName = $2, company = $3, location = $4 WHERE firstName = $5 AND lastName = $6 AND company = $7 AND location = $8',
				[
					updatedFitTestPerson.firstName,
					updatedFitTestPerson.lastName,
					updatedFitTestPerson.company,
					updatedFitTestPerson.location,
					fitTestPerson.firstName,
					fitTestPerson.lastName,
					fitTestPerson.company,
					fitTestPerson.location
				]
			);
			if (success == 0) {
				console.log('Failed Update');
			}
		}
	}

	onMount(async () => {
		testState.subscribe(async (newTestState) => {
			const id = newTestState.person.id;
			db = Database.get('sqlite:' + newTestState.database);
			const fitTestPeople: FitTestPerson[] = await db.select(
				'SELECT id, idNumber, firstName, lastName, company, location, maskDescription, maskSize from fitTestRecord where id = $1',
				[id]
			);
			const masks: Array<{ maskDescription: string }> = await db.select(
				'SELECT maskDescription FROM maskRecord'
			);
			masksArr = masks.map((mask) => mask.maskDescription);
			fitTestPerson = fitTestPeople[0];
			//replace json with something else to copy?
			updatedFitTestPerson = JSON.parse(JSON.stringify(fitTestPeople[0]));
			updatedFitTestPerson.firstName = toTitleCase(updatedFitTestPerson.firstName)
			updatedFitTestPerson.lastName = toTitleCase(updatedFitTestPerson.lastName)
			updatedFitTestPerson.company = toTitleCase(updatedFitTestPerson.company)
			updatedFitTestPerson.location = toTitleCase(updatedFitTestPerson.location)
		});
		const canvas = document.querySelector('canvas') as HTMLCanvasElement;

		signaturePad = new SignaturePad(canvas);
		signaturePad.addEventListener('endStroke', (e) => {
			signed = !signaturePad.isEmpty();
		});

	});

	function clear() {
		signed = false;
		signaturePad.clear();
	}

</script>
<Breadcrumb active={3}/>
<div class="py-4">
	<p class="text-gray-500 text-center py-2 text-xl">
		This page must be completed by the client.
	</p>
	<p class="text-gray-500 text-center pb2 text-m">Please correct each field, click confirm to lock in details and sign.</p>
	<div class="text-left ml-20">
		<a href="/people"><button class="border py-2 px-6 rounded">Back</button></a>
	</div>
</div>
<div class="md:px-20">
	<div class="md:grid md:grid-cols-3 md:gap-6">
		<div class="mt-5 md:col-span-3 md:mt-0">
			<form action="#" method="POST">
				<div class="overflow-hidden shadow sm:rounded-md">
					<div class="bg-white px-4 py-5 sm:p-6">
						<div class="grid grid-cols-7 gap-6">
							<div class="col-span-3">
								<label for="first-name" class="block text-sm font-medium text-gray-700"
									>First name</label
								>
								<input
									disabled={linesDisabled[0]}
									bind:value={updatedFitTestPerson.firstName}
									type="text"
									name="first-name"
									id="first-name"
									autocomplete="given-name"
									class="disabled:bg-green-200 disabled:text-gray-600 mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
								/>
							</div>
							<div class="col-span-3">
								<label for="last-name" class="block text-sm font-medium text-gray-700"
									>Last name</label
								>
								<input
									disabled={linesDisabled[0]}
									bind:value={updatedFitTestPerson.lastName}
									type="text"
									name="last-name"
									id="last-name"
									autocomplete="family-name"
									class="disabled:bg-green-200 disabled:text-gray-600 mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
								/>
							</div>
							<div class="col-span-1 flex flex-col justify-end items-center">
								<button
									on:click={() => toggleLine(0)}
									type="button"
									class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
								>
									{linesDisabled[0] ? 'Unlock' : 'Confirm'}
								</button>
							</div>

							<div class="col-span-3">
								<label for="company" class="block text-sm font-medium text-gray-700">Company</label>
								<input
									disabled={linesDisabled[1]}
									bind:value={updatedFitTestPerson.company}
									type="text"
									name="company"
									id="company"
									class="disabled:bg-green-200 disabled:text-gray-600 mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
								/>
							</div>
							<div class="col-span-3">
								<label for="location" class="block text-sm font-medium text-gray-700"
									>Location</label
								>
								<input
									disabled={linesDisabled[1]}
									bind:value={updatedFitTestPerson.location}
									type="text"
									name="location"
									id="location"
									class="disabled:bg-green-200 disabled:text-gray-600 mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
								/>
							</div>
							<div class="col-span-1 flex flex-col justify-end items-center">
								<button
									on:click={() => toggleLine(1)}
									type="button"
									class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
								>
									{linesDisabled[1] ? 'Unlock' : 'Confirm'}
								</button>
							</div>

							<div class="col-span-3">
								<label for="mask" class="block text-sm font-medium text-gray-700">Mask</label>
								<select
									disabled={linesDisabled[2]}
									bind:value={updatedFitTestPerson.maskDescription}
									id="mask"
									name="mask"
									class="disabled:bg-green-300 disabled:text-gray-800 mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
								>
									{#each masksArr as mask}
										<option value={mask}>{mask}</option>
									{/each}
									<!-- <option>Alpha Solway 2530V FFP3 [100]</option>
                  <option>3M 7502 Half Mask [100]</option> -->
								</select>
							</div>

							<div class="col-span-3">
								<label for="mask-size" class="block text-sm font-medium text-gray-700"
									>Mask Size</label
								>
								<select
									disabled={linesDisabled[2]}
									bind:value={updatedFitTestPerson.maskSize}
									id="mask-size"
									name="mask-size"
									class="disabled:bg-green-300 disabled:text-gray-800 mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
								>
									<option value="ExtraSmall">Extra Small</option>
									<option value="Small">Small</option>
									<option value="Small/Medium">Small/Medium</option>
									<option value="Medium">Medium</option>
									<option value="Medium/Large">Medium/Large</option>
									<option value="Large">Large</option>
									<option value="Large/ExtraLarge">Large/Extra Large</option>
									<option value="OneSize">One Size</option>
								</select>
							</div>

							<div class="col-span-1 flex flex-col justify-end items-center">
								<button
									on:click={() => toggleLine(2)}
									type="button"
									class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
								>
									{linesDisabled[2] ? 'Unlock' : 'Confirm'}
								</button>
							</div>

							<div class="col-span-7 text-center flex flex-col items-center">
								<label for="signature" class="block text-sm font-medium text-gray-700"
									>Signature</label
								>
								<canvas id="sig-canvas" width="500px"> Get a better browser, bro. </canvas>
								<button
									type="button"
									on:click={clear}
									class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:bg-gray-300"
									>Clear Signature</button
								>
							</div>
						</div>
						<p class="text-gray-500 text-center">Please pass the laptop back to the operator.</p>
					</div>

					<div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
						<a href="/operator">
							<button
								on:click={save}
								disabled={!(linesDisabled.every((line) => line) && signed)}
								class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:bg-gray-300"
								>{linesDisabled.every((line) => line) && signed
									? 'Continue'
									: 'Confirm all information is correct and sign to continue.'}</button
							>
						</a>
					</div>
				</div>
			</form>
		</div>
	</div>
</div>
