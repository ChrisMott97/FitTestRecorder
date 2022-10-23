<script lang="ts">
	import SmallTable from '../SmallTable.svelte';
	import type { Heading } from '../types';
	import { onMount } from 'svelte';
	import { testState } from '../stores';
	import Database from 'tauri-plugin-sql-api';
	import SignaturePad from 'signature_pad';
	import { fs } from '@tauri-apps/api';
	import { documentDir, publicDir, sep } from '@tauri-apps/api/path';
	import Breadcrumb from '../Breadcrumb.svelte'
	import { invoke } from '@tauri-apps/api/tauri'

	const headings: Heading[] = [
		{ label: 'Exercise', key: 'exercise' },
		{ label: 'Factor', key: 'factor' }
	];
	const postPublicPath = `RPAVerify`;

	type ExerciseField = {
		exercise: string;
		factor: number;
	};

	type Exercise = {
		visible: {
			exercise: string;
			factor: number;
		};
	};

	let data: Exercise[] = [];
	let newData: Exercise[] = [];

	let note: string = '';

	let db: Database;
	let id = 0;

	let signaturePad: SignaturePad;
	let newSignature = false;
	let signed = false;

	let warning: boolean = false;

	async function save() {
		const imgData = signaturePad.toData();
		const newDir = postPublicPath;
		if (newSignature) {
			fs.exists(newDir, { dir: fs.BaseDirectory.Config }).then((dir) => {
				if (!dir) {
					fs.createDir(newDir, { dir: fs.BaseDirectory.Config });
				}
				fs.writeTextFile(newDir + sep + 'settings.json', JSON.stringify(imgData), {
					dir: fs.BaseDirectory.Config
				});
			});
		}
		

		const { rowsAffected } = await db.execute(
			'update fitTestRecord set note = $1, description = $2 where id = $3',
			[note, 'verified', id]
		);

		console.log('Success: ', Boolean(rowsAffected));
	}

	onMount(async () => {
		const canvas = document.querySelector('canvas') as HTMLCanvasElement;
		signaturePad = new SignaturePad(canvas);
		signaturePad.addEventListener('endStroke', (e) => {
			newSignature = true;
			signed = true;
		});
		const newDir = postPublicPath;
		fs.exists(newDir, { dir: fs.BaseDirectory.Config }).then((dir) => {
			if (!dir) {
				fs.createDir(newDir, { dir: fs.BaseDirectory.Config });
			}
			fs.exists(newDir + sep + 'settings.json', { dir: fs.BaseDirectory.Config }).then(
				(settingsExist) => {
					if (settingsExist) {
						fs.readTextFile(newDir + sep + 'settings.json', { dir: fs.BaseDirectory.Config }).then(
							(value) => {
								signaturePad.fromData(JSON.parse(value));
								signed = !signaturePad.isEmpty();
							}
						);
					}
				}
			);
		});

		testState.subscribe(async (newTestState) => {
			if (newTestState.person.id == 0) {
				return;
			}
			console.log('subscribing');
			console.log(newTestState.person.id);
			id = newTestState.person.id;
			db = await Database.load('sqlite:' + newTestState.database);

			const numExercisesResult: {
				numExercises: number;
				overallFf: number;
				note: string;
				avgAmbient: number;
			}[] = await db.select('SELECT * FROM fitTestRecord where id = $1', [id]);
			invoke('get_avg_ambient').then((res)=>{
				console.log(res)
			})


			console.log(numExercisesResult);
			const numExercises = numExercisesResult[0].numExercises;

			const exercises: Exercise[] = new Array(numExercises);
			for (let i = 0; i < numExercises; i++) {
				const exercise: ExerciseField[] = await db.select(
					`select exercise${i + 1} as exercise, fitFactor${
						i + 1
					} as factor from fitTestRecord where id = $1`,
					[id]
				);
				exercise[0].exercise = exercise[0].exercise.toLowerCase();
				exercises[i] = { visible: exercise[0] };
			}

			// exercises.push({
			// 	visible: { exercise: 'Avg Ambient', factor: numExercisesResult[0].avgAmbient }
			// })

			//avg ambient returning as null, may have to migrate to

			exercises.push({
				visible: { exercise: 'Overall Fit Factor', factor: numExercisesResult[0].overallFf }
			});

			note = numExercisesResult[0].note;

			data = exercises;

			for (const e of exercises) {
				if (e.visible.factor > 100000) {
					warning = true;
					break;
				}
			}
			console.log(exercises);
		});
	});

	function clear() {
		signed = false;
		signaturePad.clear();
	}
</script>
<Breadcrumb active={4}/>
<div class="py-4">
	<p class="text-gray-500 text-center py-2 text-xl">
		This page must be completed by the operator.
	</p>
	<p class="text-gray-500 text-center py-2 text-m">Please ensure all notes are written and signed for.</p>
	<div class="text-left ml-20">
		<a href="/person"><button class="border py-2 px-6 rounded">Back</button></a>
	</div>
</div>
{#if warning}
	<div class="rounded-md bg-red-50 p-4 mx-20">
		<div class="flex">
			<div class="flex-shrink-0">
				<!-- Heroicon name: solid/exclamation -->
				<svg
					class="h-5 w-5 text-red-400"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 20 20"
					fill="currentColor"
					aria-hidden="true"
				>
					<path
						fill-rule="evenodd"
						d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
						clip-rule="evenodd"
					/>
				</svg>
			</div>
			<div class="ml-3">
				<h3 class="text-sm font-medium text-red-800">Attention needed!</h3>
				<div class="mt-2 text-sm text-red-700">
					<p>
						Abnormally high value detected, please ensure the appropriate additional tests have been
						done!
					</p>
				</div>
			</div>
		</div>
	</div>
{/if}
<div class="gap-4 grid grid-cols-1 md:grid-cols-2 pt-2 mx-20">
	<div class="">
		<SmallTable {headings} {data} />
	</div>
	<div class="">
		<div>
			<div class="md:grid md:grid-cols-3 md:gap-6">
				<div class="mt-5 md:col-span-3 md:mt-0">
					<form action="#" method="POST">
						<div class="overflow-hidden shadow sm:rounded-md">
							<div class="bg-white px-4 py-5 sm:p-6">
								<div class="grid grid-cols-7 gap-6">
									<div class="col-span-7">
										<label for="comment" class="block text-sm font-medium text-gray-700"
											>Notes</label
										>
										<div class="mt-1">
											<textarea
												rows="4"
												name="comment"
												id="comment"
												bind:value={note}
												class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
											/>
										</div>
									</div>
									<div class="col-span-7">
										<label for="signature" class="block text-sm font-medium text-gray-700"
											>Signature</label
										>
										<canvas id="sig-canvas" width="300px"> Error. </canvas>
										<button
											type="button"
											on:click={clear}
											class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:bg-gray-300"
											>Clear Signature</button
										>
									</div>
								</div>
							</div>
							<div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
								<a href="/">
									<button
										on:click={save}
										disabled={!signed}
										class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:bg-gray-300"
										>{signed
											? 'Complete'
											: 'Please sign to complete'}</button
									>
								</a>
							</div>
						</div>
					</form>
				</div>
			</div>
		</div>
	</div>
</div>
