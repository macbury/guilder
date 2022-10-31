import React from 'react'
import SelectPicker from 'rsuite/SelectPicker';

export interface IIntegrationKindSelectProps {

}

const kinds = [
  { label: "Obligacje Skarbowe PKOBP", value: "PKOBP" }
];

export default function IntegrationKindSelect(props : IIntegrationKindSelectProps) {
  return (
    <SelectPicker
      data={kinds}
      {...props}
    />
  )
}
