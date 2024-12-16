<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import ProgressBar from 'primevue/progressbar'
import Card from 'primevue/card'
import { open } from '@tauri-apps/api/dialog'
import { appConfigDir } from '@tauri-apps/api/path'
import { shell } from '@tauri-apps/api'

const currentStep = ref(1)
const progress = ref(0)
const installPath = ref("")
const progressMessage = ref("Status: Wartet auf User 🌙")

// Step 1: Select Installation Folder
async function selectFolder() {
  progressMessage.value = "Installationsordner wird ausgewählt..."
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await appConfigDir(),
  });
  if (selected) {
    installPath.value = selected as string
    progressMessage.value = `Installationsordner ausgewählt: ${installPath.value}`
    nextStep()
  } else {
    progressMessage.value = "Ordnerauswahl abgebrochen!"
  }
}

// Step 2: Copy QWM Folder
async function copyQWMFolder() {
  progressMessage.value = "QWM-Ordner wird kopiert..."
  console.log(installPath.value)
  try {
    await invoke('copy_qwm_folder', { targetPath: installPath.value })
    progressMessage.value = "QWM-Ordner erfolgreich kopiert!"
    nextStep()
  } catch (e) {
    console.error("Fehler beim Kopieren:", e)
    progressMessage.value = `Fehler: ${e}`
  }
}

// Step 3: Install PostgreSQL
async function installPostgres() {
  progressMessage.value = "PostgreSQL wird installiert..."
  try {
    await invoke('install_postgres', { installationPath: installPath.value })
    progressMessage.value = "PostgreSQL erfolgreich installiert!"
    nextStep()
  } catch (e) {
    console.error("Fehler bei PostgreSQL-Installation:", e)
    progressMessage.value = `Fehler: ${e}`
  }
}

// Step 4: Configure Database (Migration Script Execution)
async function configureDatabase() {
  progressMessage.value = "Datenbank wird konfiguriert..."
  try {
    await invoke('configure_database', { installationPath: installPath.value })
    progressMessage.value = "Datenbank erfolgreich konfiguriert!"
    nextStep()
  } catch (e) {
    console.error("Fehler bei Datenbank-Konfiguration:", e)
    progressMessage.value = `Fehler: ${e}`
  }
}

// Step 5: Configure Service
async function configureService() {
  progressMessage.value = "Dienst wird mit NSSM konfiguriert..."
  try {
    await invoke('configure_service', { installationPath: installPath.value })
    progressMessage.value = "Dienst erfolgreich konfiguriert!"
    nextStep()
  } catch (e) {
    console.error("Fehler bei Dienstkonfiguration:", e)
    progressMessage.value = `Fehler: ${e}`
  }
}

async function startService() {
  progressMessage.value = "Dienst wird gestartet..."
  try {
    await invoke('start_service', { installationPath: installPath.value })
    progressMessage.value = "Dienst erfolgreich gestartet!"
    nextStep()
  } catch (e) {
    console.error("Fehler beim Dienststart:", e)
    progressMessage.value = `Fehler: ${e}`
  }
}

async function openQWMInBrowser() {
  await shell.open("http://localhost:9998/erp/#/")
}

// Skip Step Functionality
async function skipStep() {
  progressMessage.value = "Schritt übersprungen."
  nextStep()
}

// Progress Control
function nextStep() {
  currentStep.value++
  progress.value = Math.round(Math.min((currentStep.value / 7) * 100, 100))
}
</script>

<template>
  <div class="w-11/12 mx-auto p-6 h-screen flex items-center justify-center">
    <Card class="">
      <template #title>QWM Installationsassistent</template>

      <template #content>
        <p>
          Willkommen beim Installationsassistenten für QWM!<br>
          Folgen Sie den Schritten, um die Installation abzuschließen.
        </p>

        <ProgressBar :value="progress" class="mt-4" />
        <p class="mt-2">{{ progressMessage }}</p>

        <div v-if="currentStep === 1" class="mt-4">
          <h3>Schritt 1: Installationsordner wählen</h3>
          <p class="mt-1">Dort wird ein Ordner namens "QWM" angelegt.</p>
          <InputText v-model="installPath" placeholder="Installationspfad (z.B.: C:\Program Files\)" class="w-full" />
          <div class="flex justify-end">
            <Button label="Ordner auswählen" icon="pi pi-folder-open" class="mt-2" @click="selectFolder" />
          </div>
        </div>

        <div v-if="currentStep === 2" class="mt-4">
          <h3>Schritt 2: QWM Source kopieren</h3>
          <p>Die QWM-Dateien werden in den Installationsordner kopiert.</p>
          <Button label="QWM-Ordner kopieren" icon="pi pi-copy" class="mt-2" @click="copyQWMFolder" />
        </div>

        <div v-if="currentStep === 3" class="mt-4">
          <h3>Schritt 3: PostgreSQL 15 installieren</h3>
          <p>Klicken Sie hier, um PostgreSQL zu installieren.</p>
          <p>Sollte PostgreSQL bereits installiert sein, klicken Sie "Überspringen".</p>
          <div class="flex justify-between">
            <Button label="PostgreSQL installieren" icon="pi pi-database" class="mt-2" @click="installPostgres" />
            <Button label="Überspringen" class="p-button-secondary mt-2" @click="skipStep" />
          </div>
        </div>

        <div v-if="currentStep === 4" class="mt-4">
          <h3>Schritt 4: Datenbank konfigurieren</h3>
          <p>Klicken Sie hier, um die Datenbank zu konfigurieren.</p>
          <div class="flex justify-between">
            <Button label="Datenbank konfigurieren" icon="pi pi-cog" class="mt-2" @click="configureDatabase" />
            <Button label="Überspringen" class="p-button-secondary mt-2" @click="skipStep" />
          </div>
        </div>

        <div v-if="currentStep === 5" class="mt-4">
          <h3>Schritt 5: Dienst konfigurieren</h3>
          <p>Klicken Sie hier, um den Windows-Dienst zu konfigurieren.</p>
          <div class="flex justify-between">
            <Button label="Dienst konfigurieren" icon="pi pi-cog" class="mt-2" @click="configureService" />
            <Button label="Überspringen" class="p-button-secondary mt-2" @click="skipStep" />
          </div>
        </div>

        <div v-if="currentStep === 6" class="mt-4">
          <h3>Schritt 6: Dienst starten</h3>
          <p>Klicken Sie hier, um den QWM Windows Dienst zu starten.</p>
          <Button label="Dienst starten" icon="pi pi-play" class="mt-2" @click="startService" />
        </div>

        <div v-if="currentStep > 6" class="mt-4 text-green-500 text-center">
          <h3 class="text-2xl font-bold">🎉 Installation abgeschlossen!</h3>
          <p class="mt-2">Die Anwendung wurde erfolgreich installiert, konfiguriert und gestartet!</p>
          <p class="mt-4">
            <Button label="Zum QWM Login" icon="pi pi-external-link" class="p-button-success p-button-lg"
              @click="openQWMInBrowser" />
          </p>
        </div>
      </template>
    </Card>
  </div>
</template>



<style scoped>
h3 {
  margin-bottom: 1rem;
}
</style>
