import { hu } from "date-fns/locale";

const mainTimeFormatLocale = {
    lastWeek: "EEEE HH:mm",
    yesterday: "'tegnap' HH:mm",
    today: "'ma' HH:mm",
    tomorrow: "'holnap' HH:mm",
    nextWeek: "dddd 'um' LT",
    other: "yyyy.MM.dd HH:mm",
};

export const locale = {
    ...hu,
    //@ts-expect-error
    formatRelative: (token) => mainTimeFormatLocale[token],
};
