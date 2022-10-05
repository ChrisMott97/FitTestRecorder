<script lang="ts">
  import Database from 'tauri-plugin-sql-api';
  import { onMount } from 'svelte';
  import { testState } from '../stores'

  let linesDisabled = [false, false, false]

  type FitTestPerson = {
    id: number,
    idNumber: string,
    firstName: string,
    lastName: string,
    company: string,
    location: string,
    maskDescription: string,
    maskSize: string
  }

  type Mask = {
    maskManufacturer: string,
    maskModel: string,
    maskStyle: string
  }

  let fitTestPerson: FitTestPerson = {
    id: 0,
    idNumber: "",
    firstName: "",
    lastName: "",
    company: "",
    location: "",
    maskDescription: "",
    maskSize: ""
  };

  let updatedFitTestPerson: FitTestPerson = {
    id: 0,
    idNumber: "",
    firstName: "",
    lastName: "",
    company: "",
    location: "",
    maskDescription: "",
    maskSize: ""
  };

  let db: Database;
  let masksArr: string[] = [];

  function toggleLine(lineIndex: number){
    linesDisabled[lineIndex] = !linesDisabled[lineIndex]
  }

  async function save(){
    const maskInfo: Mask[] = await db.select('select maskManufacturer, maskModel, maskStyle from maskRecord where maskDescription = $1', [updatedFitTestPerson.maskDescription])
    const {maskManufacturer, maskModel, maskStyle} = maskInfo[0]
    // add ffPassLevel and n95 fields update
    console.log(fitTestPerson)
    console.log(updatedFitTestPerson)
    db.execute(
      'UPDATE peopleRecord SET firstName = $1, lastName = $2, company = $3, location = $4 WHERE firstName = $5 AND lastName = $6 AND idNumber = $7 AND company = $8 AND location = $9',
      [updatedFitTestPerson.firstName, updatedFitTestPerson.lastName, updatedFitTestPerson.company, updatedFitTestPerson.location, fitTestPerson.firstName, fitTestPerson.lastName, fitTestPerson.idNumber, fitTestPerson.company, fitTestPerson.location]
    )
    db.execute(
      'UPDATE fitTestRecord SET maskSize = $1, firstName = $2, lastName = $3, company = $4, location = $5, maskManufacturer = $6, maskModel = $7, maskStyle = $8, maskDescription = $9 WHERE id = $10',
      [updatedFitTestPerson.maskSize, updatedFitTestPerson.firstName, updatedFitTestPerson.lastName, updatedFitTestPerson.company, updatedFitTestPerson.location, maskManufacturer, maskModel, maskStyle, updatedFitTestPerson.maskDescription, fitTestPerson.id]
    );

    // isn't working 
    
  }

  onMount(async () => {
    testState.subscribe(async newTestState => {
      console.log("subscribing")
      const id = newTestState.person.id;
      db = await Database.load('sqlite:' + newTestState.database);
      const fitTestPeople: FitTestPerson[] = await db.select('SELECT id, idNumber, firstName, lastName, company, location, maskDescription, maskSize from fitTestRecord where id = $1', [id]);
      const masks: Array<{maskDescription: string}> = await db.select('SELECT maskDescription FROM maskRecord')
      masksArr = masks.map(mask => mask.maskDescription)
      fitTestPerson = fitTestPeople[0];
      updatedFitTestPerson = JSON.parse(JSON.stringify(fitTestPeople[0]));
    })
  })
</script>

<div class="mt-5 md:px-20">
  <div class="md:grid md:grid-cols-3 md:gap-6">
    <div class="mt-5 md:col-span-3 md:mt-0">
      <form action="#" method="POST">
        <div class="overflow-hidden shadow sm:rounded-md">
          <div class="bg-white px-4 py-5 sm:p-6">
            <div class="grid grid-cols-7 gap-6">
              <div class="col-span-3">
                <label for="first-name" class="block text-sm font-medium text-gray-700">First name</label>
                <input disabled='{linesDisabled[0]}' bind:value={updatedFitTestPerson.firstName} type="text" name="first-name" id="first-name" autocomplete="given-name" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-3">
                <label for="last-name" class="block text-sm font-medium text-gray-700">Last name</label>
                <input disabled='{linesDisabled[0]}' bind:value="{updatedFitTestPerson.lastName}" type="text" name="last-name" id="last-name" autocomplete="family-name" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-1 flex flex-col justify-end items-center">
                <button on:click={()=>toggleLine(0)} type="button" class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  {linesDisabled[0] ? "Unlock" : "Confirm"}
                </button>
              </div>

              <div class="col-span-3">
                <label for="company" class="block text-sm font-medium text-gray-700">Company</label>
                <input disabled='{linesDisabled[1]}' bind:value="{updatedFitTestPerson.company}" type="text" name="company" id="company" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-3">
                <label for="location" class="block text-sm font-medium text-gray-700">Location</label>
                <input disabled='{linesDisabled[1]}' bind:value="{updatedFitTestPerson.location}" type="text" name="location" id="location" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm">
              </div>
              <div class="col-span-1 flex flex-col justify-end items-center">
                <button on:click={()=>toggleLine(1)} type="button" class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  {linesDisabled[1] ? "Unlock" : "Confirm"}
                </button>
              </div>

              <div class="col-span-3">
                <label for="mask" class="block text-sm font-medium text-gray-700">Mask</label>
                <select disabled='{linesDisabled[2]}' bind:value="{updatedFitTestPerson.maskDescription}" id="mask" name="mask" class="mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm">
                  {#each masksArr as mask}
                    <option value="{mask}">{mask}</option>
                  {/each}
                  <!-- <option>Alpha Solway 2530V FFP3 [100]</option>
                  <option>3M 7502 Half Mask [100]</option> -->
                </select>
              </div>

              <div class="col-span-3">
                <label for="mask-size" class="block text-sm font-medium text-gray-700">Mask Size</label>
                <select disabled='{linesDisabled[2]}' bind:value="{updatedFitTestPerson.maskSize}" id="mask-size" name="mask-size" class="mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm">
                  <option value="OneSize">OneSize</option>
                  <option value="Small">Small</option>
                  <option value="Medium">Medium</option>
                  <option value="Large">Large</option>
                </select>
              </div>

              <div class="col-span-1 flex flex-col justify-end items-center">
                <button on:click={()=>toggleLine(2)} type="button" class="inline-flex items-center px-4 py-1.5 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  {linesDisabled[2] ? "Unlock" : "Confirm"}
                </button>
              </div>

              <div class="col-span-7">
                <label for="signature" class="block text-sm font-medium text-gray-700">Signature</label>
                <canvas id="sig-canvas" class="w-full h-40">
                  Get a better browser, bro.
                </canvas>
              </div>
              
            </div>
            <p class="text-gray-500 text-center">Please pass the laptop back to the operator.</p>
          </div>
          
          <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
            <a href="/operator">
              <button on:click={save} disabled='{!linesDisabled.every((line)=>(line))}' class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:bg-gray-300">Save</button>
            </a>
          </div>
        </div>
      </form>
    </div>
  </div>
</div>
