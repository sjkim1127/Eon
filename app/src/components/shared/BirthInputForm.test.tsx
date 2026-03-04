import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { BirthInputForm } from "./BirthInputForm";
import type { BirthData } from "../../types";

const mockBirthData: BirthData = {
  year: 1990,
  month: 1,
  day: 1,
  hour: 12,
  minute: 0,
  lat: 37.5665,
  lon: 126.978,
  is_lunar: false,
  is_leap_month: false,
  timezone: "Asia/Seoul",
};

describe("BirthInputForm", () => {
  it("renders in compact mode without wrapper title", () => {
    render(
      <BirthInputForm
        birthData={mockBirthData}
        setBirthData={() => {}}
        selectedCity="서울"
        onCitySelect={() => {}}
        isMale={true}
        setIsMale={() => {}}
        isDST={false}
        loading={false}
        onAnalysis={() => {}}
        sajuReport={null}
        compact
        submitLabel="궁합 분석 시작"
        hideUnknownTime
      />
    );

    expect(screen.getByText("궁합 분석 시작")).toBeInTheDocument();
  });

  it("renders in full mode with wrapper title", () => {
    render(
      <BirthInputForm
        birthData={mockBirthData}
        setBirthData={() => {}}
        selectedCity="서울"
        onCitySelect={() => {}}
        isMale={true}
        setIsMale={() => {}}
        isDST={false}
        loading={false}
        onAnalysis={() => {}}
        sajuReport={null}
      />
    );

    expect(screen.getByText("출생 정보 입력")).toBeInTheDocument();
  });
});
