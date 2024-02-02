module.exports = {
    root: true,
    env: { browser: true, es2020: true },
    parser: '@typescript-eslint/parser', // Specifies the ESLint parser
    parserOptions: {
        ecmaVersion: 2020, // Allows for the parsing of modern ECMAScript features
        sourceType: 'module', // Allows for the use of imports
        extraFileExtensions: ['.astro'],
        ecmaFeatures: {
            jsx: true, // Allows for the parsing of JSX
        },
    },
    settings: {
        react: {
            version: 'detect', // Tells eslint-plugin-react to automatically detect the version of React to use
        },
    },
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:react/recommended',
        'plugin:react-hooks/recommended',
        'plugin:import/recommended',
        'plugin:prettier/recommended',
        'plugin:astro/recommended',
    ],
    plugins: ['react-refresh'],
    ignorePatterns: ['dist', '.eslintrc.cjs', 'rust.d.ts'],
    rules: {
        'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
    },
    overrides: [
        // Configuration for mjs,cjs files
        {
            files: ['*.mjs', '*.cjs'],
            extends: ['plugin:prettier/recommended'],
            rules: {
                'import/no-extraneous-dependencies': 'off', // mjs is only used by Astro for configuration, false positive
                'import/no-unresolved': 'off', // Also false positive with mjs file
            },
        },
        // Configuration for TypeScript files
        {
            parser: '@typescript-eslint/parser',
            files: ['*.ts', '*.tsx'],

            plugins: ['@typescript-eslint', 'react', 'unused-imports', 'tailwindcss', 'simple-import-sort'],
            extends: ['plugin:tailwindcss/recommended', 'airbnb-typescript/base', 'plugin:prettier/recommended'],
            parserOptions: {
                project: './tsconfig.json',
            },
            rules: {
                'import/extensions': [
                    'error',
                    'ignorePackages',
                    {
                        js: 'never',
                        jsx: 'never',
                        ts: 'never',
                        tsx: 'never',
                        '': 'never',
                    },
                ], // Avoid missing file extension errors when using '@/' alias
                'react/destructuring-assignment': 'off', // Vscode doesn't support automatically destructuring, it's a pain to add a new variable
                'react/require-default-props': 'off', // Allow non-defined react props as undefined
                'react/jsx-props-no-spreading': 'off', // _app.tsx uses spread operator and also, react-hook-form
                '@typescript-eslint/comma-dangle': 'off', // Avoid conflict rule between Eslint and Prettier
                '@typescript-eslint/consistent-type-imports': 'error', // Ensure `import type` is used when it's necessary
                'import/prefer-default-export': 'off', // Named export is easier to refactor automatically
                'tailwindcss/classnames-order': [
                    'warn',
                    {
                        officialSorting: true,
                    },
                ], // Follow the same ordering as the official plugin `prettier-plugin-tailwindcss`
                'simple-import-sort/imports': 'error', // Import configuration for `eslint-plugin-simple-import-sort`
                'simple-import-sort/exports': 'error', // Export configuration for `eslint-plugin-simple-import-sort`
                '@typescript-eslint/no-unused-vars': 'off',
                'unused-imports/no-unused-imports': 'error',
                'unused-imports/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
            },
        },
        // Configuration for Astro
        {
            files: ['*.astro'],
            plugins: ['@typescript-eslint', 'react', 'unused-imports', 'tailwindcss', 'simple-import-sort'],
            extends: ['plugin:tailwindcss/recommended', 'airbnb-typescript/base', 'plugin:prettier/recommended'],
            parser: 'astro-eslint-parser',
            parserOptions: {
                parser: '@typescript-eslint/parser',
            },
            rules: {
                'import/extensions': [
                    'error',
                    'ignorePackages',
                    {
                        js: 'never',
                        jsx: 'never',
                        ts: 'never',
                        tsx: 'never',
                        '': 'never',
                    },
                ], // Avoid missing file extension errors in .astro files
                'import/no-unresolved': [
                    'error',
                    {
                        ignore: ['@/*'],
                    },
                ], // Disable no-unresolved rule for .astro files
                'react/jsx-filename-extension': [1, { extensions: ['.astro'] }], // Accept jsx in astro files
                'react/destructuring-assignment': 'off', // Vscode doesn't support automatically destructuring, it's a pain to add a new variable
                'react/require-default-props': 'off', // Allow non-defined react props as undefined
                'react/jsx-props-no-spreading': 'off', // _app.tsx uses spread operator and also, react-hook-form
                '@typescript-eslint/comma-dangle': 'off', // Avoid conflict rule between Eslint and Prettier
                '@typescript-eslint/consistent-type-imports': 'error', // Ensure `import type` is used when it's necessary
                'import/prefer-default-export': 'off', // Named export is easier to refactor automatically
                'tailwindcss/classnames-order': [
                    'warn',
                    {
                        officialSorting: true,
                    },
                ], // Follow the same ordering as the official plugin `prettier-plugin-tailwindcss`
                'simple-import-sort/imports': 'error', // Import configuration for `eslint-plugin-simple-import-sort`
                'simple-import-sort/exports': 'error', // Export configuration for `eslint-plugin-simple-import-sort`
                '@typescript-eslint/no-unused-vars': 'off',
                'unused-imports/no-unused-imports': 'error',
                'unused-imports/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
            },
            globals: {
                Astro: 'readonly',
            },
        },
    ],
};
