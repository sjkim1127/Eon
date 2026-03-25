import { useAppStore } from '../store/useAppStore';

export function useBirthForm() {
  const birthData = useAppStore(state => state.birthData);
  const setBirthData = useAppStore(state => state.setBirthData);
  const selectedCity = useAppStore(state => state.selectedCity);
  const setSelectedCity = useAppStore(state => state.setSelectedCity);
  const isMale = useAppStore(state => state.isMale);
  const setIsMale = useAppStore(state => state.setIsMale);


  const handleCitySelect = (city: { name: string; lat: number; lon: number; timezone: string }) => {
    setSelectedCity(city.name);
    setBirthData((prev: any) => ({ ...prev, lat: city.lat, lon: city.lon, timezone: city.timezone }));
  };

  return {
    birthData, setBirthData,
    selectedCity, handleCitySelect,
    isMale, setIsMale,
  };
}
