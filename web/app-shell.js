import { invoke } from '@tauri-apps/api/core';
import { getVersion } from '@tauri-apps/api/app';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

const isTauri = !!window.__TAURI_INTERNALS__;

async function checkForUpdates({ interactive = false } = {}) {
  if (!isTauri) return { available: false, reason: 'browser' };
  try {
    const update = await check();
    if (!update) {
      if (interactive) alert('현재 최신 버전입니다.');
      return { available: false };
    }

    const ok = confirm(
      `GTNH Flow Chart ${update.version} 업데이트가 있습니다.\n\n` +
      `${update.body || ''}\n\n지금 업데이트할까요?`
    );
    if (!ok) return { available: true, installed: false, version: update.version };

    await update.downloadAndInstall();
    await relaunch();
    return { available: true, installed: true, version: update.version };
  } catch (error) {
    console.warn('[GTNH App] update check skipped/failed:', error);
    if (interactive) {
      alert('업데이트 확인에 실패했습니다.\n\n' + String(error));
    }
    return { available: false, error: String(error) };
  }
}

async function saveProject(name, data) {
  if (!isTauri) throw new Error('Native app에서만 사용할 수 있습니다.');
  return invoke('save_project', { name, data });
}

async function loadProject(name) {
  if (!isTauri) throw new Error('Native app에서만 사용할 수 있습니다.');
  return invoke('load_project', { name });
}

async function listProjects() {
  if (!isTauri) return [];
  return invoke('list_projects');
}

async function deleteProject(name) {
  if (!isTauri) throw new Error('Native app에서만 사용할 수 있습니다.');
  return invoke('delete_project', { name });
}

async function exportBackup() {
  if (!isTauri) throw new Error('Native app에서만 사용할 수 있습니다.');
  return invoke('export_backup');
}

window.GTNH_APP = Object.freeze({
  isDesktop: isTauri,
  version: () => getVersion(),
  checkForUpdates: () => checkForUpdates({ interactive: true }),
  saveProject,
  loadProject,
  listProjects,
  deleteProject,
  exportBackup,
});

// 앱 실행 시 자동 업데이트 확인.
if (isTauri) {
  setTimeout(() => checkForUpdates({ interactive: true }), 700);
}
