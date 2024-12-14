<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import ProgressBar from 'primevue/progressbar'
import Card from 'primevue/card'
import { open } from '@tauri-apps/api/dialog';
import { appConfigDir } from '@tauri-apps/api/path';

const currentStep = ref(1)
const progress = ref(0)
const installPath = ref("")
const dbConfigured = ref(false)
const serviceConfigured = ref(false)
const stepDone = ref(false)
const progressMessage = ref("Status: Wartet auf User 🌙")

async function selectFolder() {
  progressMessage.value = "Installationsordner wird ausgewählt..."
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await appConfigDir(),
  });
  if (selected === null) {
    return
  } else {
    installPath.value = selected.at(0) ?? "C:\\interSalesAG\\QWM"
  }
  nextStep()
}

async function unzipFiles() {
  progressMessage.value = "Installationsdateien werden entpackt..."
  await invoke('unzip_files', { installationPath: installPath.value })
  .catch((error) => {
    console.error(error)
    return
  })

  nextStep()
}

async function installPostgres() {
  progressMessage.value = "PostgreSQL wird installiert..."
  await invoke('install_postgres')
  dbConfigured.value = true
  nextStep()
}

async function configureService() {
  progressMessage.value = "Dienst wird mit NSSM konfiguriert..."
  await invoke('configure_service', { path: installPath.value })
  serviceConfigured.value = true
  nextStep()
}

function nextStep() {
  currentStep.value++
  progress.value = (currentStep.value / 4) * 100
}

function stepBack() {
  if (currentStep.value > 1) {
    currentStep.value--
    progress.value = ((currentStep.value - 1) / 4) * 100
  }
}
</script>

<template>
  <div class="w-11/12 mx-auto p-6 h-screen flex items-center justify-center">
    <!-- Installation Card Container -->
    <Card class="">
      <template #title>QWM Installationsassistent</template>

      <template #content>
        <p>Willkommen beim Installationsassistenten vom QWM! Folgen Sie den einzelnen Schritten, um die Installation
          abzuschließen.</p>

        <!-- Progress Display -->
        <ProgressBar :value="progress" class="mt-4" />
        <p class="mt-2">{{ progressMessage }}</p>

        <!-- Step 1: Choose Installation Folder -->
        <div v-if="currentStep === 1" class="mt-4">
          <h3>Schritt 1: QWM Installationsordner wählen</h3>
          <InputText v-model="installPath" placeholder="Installationspfad (z.B.: C:\interSalesAG\QWM)" class="w-full" />
          <div class="flex justify-between">
            <Button label="Ordner Auswählen" icon="pi pi-folder-open" class="mt-2" @click="selectFolder" />
            <Button label="Nächster Schritt" :disabled="!stepDone" icon="pi pi-folder-open" class="mt-2"
              @click="stepBack" />
          </div>

        </div>

        <!-- Step 2: Unzip Files -->
        <div v-if="currentStep === 2" class="mt-4">
          <h3>Schritt 2: Dateien entpacken</h3>
          <p>Die erforderlichen Dateien werden nun in den Installationsordner entpackt.</p>
          <Button label="Dateien entpacken" icon="pi pi-file-zip" class="mt-2" @click="unzipFiles" />
        </div>

        <!-- Step 3: Install PostgreSQL -->
        <div v-if="currentStep === 3" class="mt-4">
          <h3>Schritt 3: PostgreSQL installieren</h3>
          <p>Klicken Sie hier, um PostgreSQL zu installieren und die Datenbank einzurichten.</p>
          <Button label="PostgreSQL installieren" icon="pi pi-database" class="mt-2" @click="installPostgres" />
        </div>

        <!-- Step 4: Configure Service with NSSM -->
        <div v-if="currentStep === 4" class="mt-4">
          <h3>Schritt 4: Dienst konfigurieren</h3>
          <p>Der Dienst wird nun mithilfe von NSSM als Windows-Dienst konfiguriert.</p>
          <Button label="Dienst konfigurieren" icon="pi pi-cog" class="mt-2" @click="configureService" />
        </div>

        <!-- Completion Message -->
        <div v-if="currentStep > 4" class="mt-4 text-green-500">
          <h3>Installation abgeschlossen</h3>
          <p>Die Anwendung wurde erfolgreich installiert und konfiguriert!</p>
        </div>
      </template>
    </Card>
  </div>
</template>

<style scoped>
/* Additional styling for layout and spacing */
</style>
