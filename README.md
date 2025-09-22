# Vivace
<img width="208" height="208" alt="Vivace" src="https://github.com/user-attachments/assets/8ba8384a-6a62-4c98-acac-6c67cad173e8" />

Vivace is a **customizable lock screen** built with **React + TypeScript + Vite**, powered by **Tauri** for native desktop integration, primarily to show different calendar dates.
It runs as a lightweight desktop app and stores user preferences in `%APPDATA%\Vivace\settings.json`.

I use it as my personal lockscreen, because I don't like Windows `¯\_(ツ)_/¯`

Plus, I wanted to learn a bit of Rust

---

I don't plan to add anything else since I have everything I want (unless I get bored, I guess), but contributions are gladly welcome.

## Preview

![vivace_preview](https://github.com/user-attachments/assets/665ceaa6-9710-444c-9441-a239a586217d)

#### Multi-Calendar Support
<img width="468" alt="image" src="https://github.com/user-attachments/assets/cb2e0f67-d013-4cb4-aa9e-d395e4fc691d" />

**Calendars**
- Gregorian (default)
- French Revolutionary
- Julian
- Buddhist
- Hebrew

(Feel free to suggest other calendars)

## For Development

Clone and install dependencies:

```bash
git clone https://github.com/willnjohnson/Vivace.git
cd Vivace
npm install
```

Run in development mode with hot reload:

```bash
npm run tauri dev
```

---

## Building for Release

To create a production build and installer:

```bash
npm run tauri build
```

This will produce a platform-specific bundle under:

```
src-tauri/target/release/bundle/
```

- **Windows:** `.msi` installer (recommended for distribution)
- **macOS:** `.dmg` image
- **Linux:** `.deb`, `.AppImage`, or other packages

**NOTE:** I've only tested the build for Windows.

---

## Configuration

Vivace reads its configuration from:

```
%APPDATA%\Vivace\settings.json
```

Example `settings.json`:

```jsonc
{
  "password": "hunter2",
  "background_type": "gradient",
  "background_value": "linear-gradient(135deg, #579945 0%, #764ba2 100%)",
  "avatar_path": "/fox_profile.png",
  "enabled_calendars": [
    "french_revolutionary",
    "gregorian",
    "julian",
    "buddhist",
    "jewish"
  ],
  "timeout_minutes": 1,
  "hotkey_combination": "Alt+L",
  "auto_lock_enabled": null,
  "auto_lock_minutes": null,
  "show_seconds": true,
  "date_format": "military",
  "theme": null,
  "sound_enabled": null,
  "sound_file": null
}
```

Modify this file to customize Vivace.

---

## Tech Stack

- [React](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/)
- [Vite](https://vitejs.dev/) for lightning-fast HMR
- [Tauri](https://tauri.app/) for cross-platform desktop bundling
- [ESLint](https://eslint.org/) with TypeScript rules for code quality

---

## Linting & Code Quality

Vivace uses ESLint with type-aware rules. Example config snippet:

```js
export default tseslint.config([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      ...tseslint.configs.recommendedTypeChecked,
      ...tseslint.configs.stylisticTypeChecked,
    ],
    languageOptions: {
      parserOptions: {
        project: ['./tsconfig.node.json', './tsconfig.app.json'],
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
])
```

You can also enable [eslint-plugin-react-x](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-x) and [eslint-plugin-react-dom](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-dom) for stricter React rules.

---

## License

MIT – feel free to modify and use Vivace in your own projects.
