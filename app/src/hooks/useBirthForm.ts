import { useAppStore } from '../store/useAppStore';

export function useBirthForm() {
  const birthData = useAppStore(state => state.birthData);
  const setBirthData = useAppStore(state => state.setBirthData);
  const selectedCity = useAppStore(state => state.selectedCity);
  const setSelectedCity = useAppStore(state => state.setSelectedCity);
  const isMale = useAppStore(state => state.isMale);
  const setIsMale = useAppStore(state => state.setIsMale);

  const birthData2 = useAppStore(state => state.birthData2);
  const setBirthData2 = useAppStore(state => state.setBirthData2);
  const selectedCity2 = useAppStore(state => state.selectedCity2);
  const setSelectedCity2 = useAppStore(state => state.setSelectedCity2);
  const isMale2 = useAppStore(state => state.isMale2);
  const setIsMale2 = useAppStore(state => state.setIsMale2);

  const handleCitySelect = (city: { name: string; lat: number; lon: number; timezone: string }) => {
    setSelectedCity(city.name);
    setBirthData((prev) => ({ ...prev, lat: city.lat, lon: city.lon, timezone: city.timezone }));
  };

  const handleCitySelect2 = (city: { name: string; lat: number; lon: number; timezone: string }) => {
    setSelectedCity2(city.name);
    setBirthData2((prev) => ({ ...prev, lat: city.lat, lon: city.lon, timezone: city.timezone }));
  };

  return {
    birthData, setBirthData,
    selectedCity, handleCitySelect,
    isMale, setIsMale,
    birthData2, setBirthData2,
    selectedCity2, handleCitySelect2,
    isMale2, setIsMale2,
  };
}
