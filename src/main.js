// Frontend entry (Svelte/Yew suggested)
// Palette: absolute black, signal red, bone white

import { invoke } from '@tauri-apps/api/tauri'

let result = ''

async function onSubmit(query) {
  result = await invoke('run_sidecar', { input: query })
  document.getElementById('output').innerText = result
}

document.getElementById('form').onsubmit = (e) => {
  e.preventDefault()
  const q = document.getElementById('query').value
  onSubmit(q)
}

// CSS (inline for illustration)
document.body.style.background = "#000"
document.body.style.color = "#F8F8F1"
document.getElementById("output").style.borderColor = "#FF0022"
