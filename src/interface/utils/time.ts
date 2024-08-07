import moment from 'moment-timezone';

export const formatMomentDateWithTimezone = (date: Date, timezone: string): string => {
    return moment(date).tz(timezone).format('YYYY-MM-DD HH:mm:ss Z');
};

export const formatMomentDate = (date: Date, timezone: string): string => {
    return moment(date).tz(timezone).format('YYYY-MM-DD HH:mm:ss');
};