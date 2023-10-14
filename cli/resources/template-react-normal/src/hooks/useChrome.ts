/**
 * TypeScript cannot recognize "chrome" object.
 * This hook prevents to write tons of "@ts-expect-error" or "@ts-ignore" in code.
 */ 

export default function useChrome() {
    // @ts-expect-error ---- Chrome API
    return chrome
}