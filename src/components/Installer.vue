<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import ProgressBar from 'primevue/progressbar'
import Card from 'primevue/card'
import { open } from '@tauri-apps/api/dialog'
import { appConfigDir } from '@tauri-apps/api/path'

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
  progressMessage.value = "PostgreSQL wird installiert und Datenbank wird konfiguriert..."
  try {
    await invoke('install_postgres', { installationPath: installPath.value })
    progressMessage.value = "PostgreSQL erfolgreich installiert und konfiguriert!"
    nextStep()
  } catch (e) {
    console.error("Fehler bei PostgreSQL-Installation:", e)
    progressMessage.value = `Fehler: ${e}`
  }
}

// Step 4: Configure Service with NSSM
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

function nextStep() {
  currentStep.value++
  progress.value = (currentStep.value / 4) * 100
}
</script>

<template>
  <div class="w-11/12 mx-auto p-6 h-screen flex items-center justify-center">
    <Card class="">
      <template #title>QWM Installationsassistent</template>

      <template #content>
        <p>Willkommen beim Installationsassistenten für QWM! Folgen Sie den Schritten, um die Installation
          abzuschließen.</p>

        <ProgressBar :value="progress" class="mt-4" />
        <p class="mt-2">{{ progressMessage }}</p>

        <div v-if="currentStep === 1" class="mt-4">
          <h3>Schritt 1: Installationsordner wählen</h3>
          <InputText v-model="installPath" placeholder="Installationspfad (z.B.: C:\interSalesAG\QWM)" class="w-full" />
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
          <h3>Schritt 3: PostgreSQL installieren</h3>
          <p>Klicken Sie hier, um PostgreSQL zu installieren und die QWM-Datenbank einzurichten.</p>
          <Button label="PostgreSQL installieren" icon="pi pi-database" class="mt-2" @click="installPostgres" />
        </div>

        <div v-if="currentStep === 4" class="mt-4">
          <h3>Schritt 4: Dienst konfigurieren</h3>
          <p>Klicken Sie hier, um den Windows-Dienstes zu konfigurieren.</p>
          <Button label="Dienst konfigurieren" icon="pi pi-cog" class="mt-2" @click="configureService" />
        </div>

        <div v-if="currentStep > 4" class="mt-4 text-green-500">
          <h3>Installation abgeschlossen!</h3>
          <p>Die Anwendung wurde erfolgreich installiert und konfiguriert!</p>
          <p>Die Anwendung kann jetzt gestartet werden. QWM liegt im Ordner: {{ installPath }}</p>
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
