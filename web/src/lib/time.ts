import { hu } from "date-fns/locale";

const mainTimeFormatLocale = {
    lastWeek: "'Múlt hét' EEEE HH:mm",
    yesterday: "'Tegnap' HH:mm",
    today: "'Ma' HH:mm",
    tomorrow: "'Holnap' HH:mm",
    nextWeek: "dddd 'um' LT",
    other: "yyyy.MM.dd HH:mm",
};

export const tzOffsetMs = new Date().getTimezoneOffset() * 60 * 1000;
export const locale = {
    ...hu,
    //@ts-expect-error
    formatRelative: (token) => mainTimeFormatLocale[token],
};
