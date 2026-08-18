# GTNH Flow Chart 데스크톱 앱 뼈대 — v18.7.38

현재 v18.7.8 HTML에 v18.7.36 → v18.7.37 → v18.7.38 패치를 순서대로 통합한 화면을
Tauri 2 데스크톱 앱으로 감싼 프로젝트입니다.

## 목표 구조

- 프로그램: Windows 설치형 앱 (`GTNH Flow Chart`)
- 프로젝트 데이터: 프로그램 파일과 분리해 OS의 앱 데이터 폴더에 저장
- 업데이트: GitHub Releases의 `latest.json`을 확인해 앱에서 자동 업데이트
- 개발 패치: `dev-patches/`에 보관하지만 실제 사용자 앱은 통합 빌드를 배포
- 앱 업데이트로 프로젝트 JSON을 덮어쓰지 않음

## 폴더

- `web/index.html` : 현재 플로우차트 UI 통합본
- `web/app-shell.js` : Tauri 브리지 + 실행 시 업데이트 확인
- `src-tauri/` : Windows 앱/Rust 백엔드
- `.github/workflows/release.yml` : GitHub Releases 자동 빌드/배포
- `dev-patches/` : 기존 개발 패치 보관본

## 아직 한 번만 설정해야 하는 것

`src-tauri/tauri.conf.json`에서 아래 2개를 실제 값으로 교체합니다.

1. `yuhae0410/GTNH-Flow-Chart`
2. `REPLACE_WITH_TAURI_UPDATER_PUBLIC_KEY`

업데이터 개인키는 GitHub에 올리면 안 됩니다. Tauri CLI로 키를 생성한 뒤,
GitHub Repository Secrets에 아래 이름으로 저장합니다.

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

## 개발 실행

Node.js, Rust, Windows용 WebView2/빌드 도구가 준비된 PC에서:

```powershell
npm install
npm run app:dev
```

## 설치 프로그램 빌드

```powershell
npm install
npm run app:build
```

결과물은 `src-tauri/target/release/bundle/` 아래에 생성됩니다.

## 자동 업데이트 배포 흐름

1. 기능 수정
2. `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `VERSION`의 버전을 동일하게 올림
3. Git commit/push
4. `v18.7.39` 같은 태그 push
5. GitHub Actions가 Windows 설치 파일 + updater artifact + `latest.json` 릴리스 생성
6. 설치된 다른 PC의 앱이 실행 시 `latest.json` 확인
7. 새 버전이 있으면 업데이트 설치 후 재실행

## 데이터 분리

Rust 쪽 `save_project/load_project/list_projects/delete_project` 명령은
Tauri의 앱 데이터 디렉터리 아래 `projects/` 폴더를 사용합니다.
따라서 앱 설치 파일이 교체되어도 프로젝트 데이터는 별도로 남습니다.

현재 기존 HTML의 브라우저 저장 기능을 이 네이티브 저장소로 완전히 교체한 것은 아닙니다.
다음 단계에서 기존 '저장/불러오기' 버튼을 `window.GTNH_APP`에 직접 연결하면
앱 데이터 분리가 완성됩니다.
