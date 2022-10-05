<script lang="ts">
	import SmallTable from '../SmallTable.svelte';
	import type { Heading } from '../types';
	import { onMount } from 'svelte';
	import { testState } from '../stores';
	import Database from 'tauri-plugin-sql-api';

	const headings: Heading[] = [
		{ label: 'Exercise', key: 'exercise' },
		{ label: 'Factor', key: 'factor' }
	];

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

	onMount(async () => {
		testState.subscribe(async (newTestState) => {
			console.log('subscribing');
			const id = newTestState.person.id;
			db = await Database.load('sqlite:' + newTestState.database);

			const numExercisesResult: { numExercises: number; overallFf: number; note: string }[] =
				await db.select('SELECT numExercises, overallFf, note from fitTestRecord where id = $1', [
					id
				]);
			const numExercises = numExercisesResult[0].numExercises;

			const exercises: Exercise[] = new Array(numExercises);
			for (let i = 0; i < numExercises; i++) {
				const exercise: ExerciseField[] = await db.select(
					`select exercise${i + 1} as exercise, fitFactor${
						i + 1
					} as factor from fitTestRecord where id = $1`,
					[id]
				);
				exercises[i] = { visible: exercise[0] };
			}

			exercises.push({
				visible: { exercise: 'Overall Fit Factor', factor: numExercisesResult[0].overallFf }
			});

			note = numExercisesResult[0].note;

			data = exercises;
			console.log(exercises);
		});
	});
</script>

<div class="flex flex-row gap-4 pt-5">
	<div>
		<SmallTable {headings} {data} />
	</div>
	<div>
		<div class="">
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
										<canvas id="sig-canvas" class="w-full h-50">
											Get a better browser, bro.
										</canvas>
									</div>
								</div>
							</div>
							<div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
								<button
									type="submit"
									class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
									>Save</button
								>
							</div>
						</div>
					</form>
				</div>
			</div>
		</div>
	</div>
</div>
