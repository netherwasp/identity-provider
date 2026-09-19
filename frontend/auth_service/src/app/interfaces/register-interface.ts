export interface RegisterUser {
  sub: string;
  name: string;
  given_name: string;
  family_name: string;
  middle_name: string;
  nickname: string;
  preferred_username: string;
  profile: string;
  picture: string;
  website: string;
  email: string;
  email_verified: boolean;
  gender: string;
  birthdate: string;
  zoneinfo: string;
  locale: string;
  phone_number: string;
  phone_number_verified: boolean;
  // address	JSON object	;
  updated_at: number;
}

export interface Country {
  name: string;
  iso: string;
  dial_code: string;
}

// Trim or extend as needed, or swap in a full list (e.g. libphonenumber metadata).
export const COUNTRIES: Country[] = [
  { name: 'Australia', iso: 'AU', dial_code: '+61' },
  { name: 'Canada', iso: 'CA', dial_code: '+1' },
  { name: 'China', iso: 'CN', dial_code: '+86' },
  { name: 'France', iso: 'FR', dial_code: '+33' },
  { name: 'Germany', iso: 'DE', dial_code: '+49' },
  { name: 'Hong Kong', iso: 'HK', dial_code: '+852' },
  { name: 'India', iso: 'IN', dial_code: '+91' },
  { name: 'Indonesia', iso: 'ID', dial_code: '+62' },
  { name: 'Japan', iso: 'JP', dial_code: '+81' },
  { name: 'Malaysia', iso: 'MY', dial_code: '+60' },
  { name: 'Philippines', iso: 'PH', dial_code: '+63' },
  { name: 'Singapore', iso: 'SG', dial_code: '+65' },
  { name: 'South Korea', iso: 'KR', dial_code: '+82' },
  { name: 'Thailand', iso: 'TH', dial_code: '+66' },
  { name: 'United Arab Emirates', iso: 'AE', dial_code: '+971' },
  { name: 'United Kingdom', iso: 'GB', dial_code: '+44' },
  { name: 'United States', iso: 'US', dial_code: '+1' },
  { name: 'Vietnam', iso: 'VN', dial_code: '+84' },
];
