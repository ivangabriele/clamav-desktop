import { Card } from '@components/Card'
import type { CardAction } from '@components/Card/types'
import { FileExplorer } from '@components/FileExplorer'
import type { Scanner } from '@core/Scanner/types'
import { Core } from '@core/types'
import { ScanningSpinner } from '@elements/ScanningSpinner'
import { ScreenBox } from '@layouts/ScreenBox'
import { MdClose, MdVerifiedUser } from 'react-icons/md'
import styled from 'styled-components'
import type { Promisable } from 'type-fest'

export type ScannerScreenComponentProps = Readonly<{
  canScan: boolean
  fileExplorerRootPaths: Core.Path[] | undefined
  onFileExporerChange: (selectedPaths: string[]) => Promisable<void>
  onFileExporerExpand: (expandedPath: string) => Promise<Core.Path[]>
  onScanStart: () => Promisable<void>
  onScanStop: () => Promisable<void>
  scannerState: Scanner.State | undefined
}>
export function ScannerScreenComponent({
  canScan,
  fileExplorerRootPaths,
  onFileExporerChange,
  onFileExporerExpand,
  onScanStart,
  onScanStop,
  scannerState,
}: ScannerScreenComponentProps) {
  if (scannerState?.module_status === Core.ModuleStatus.Running) {
    return (
      <ScreenBox isCentered>
        <ScanningCancelButton onClick={onScanStop}>
          <MdClose size={32} />
        </ScanningCancelButton>

        <ScanningSpinner size={128} />
        <ScanningStepText>Scanning files...</ScanningStepText>
        <ScanningTargetText>{scannerState.currently_scanned_file_path}</ScanningTargetText>
      </ScreenBox>
    )
  }

  const scanActions: CardAction[] = [
    {
      callback: onScanStart,
      isDisabled: !canScan,
      label: 'Scan',
    },
  ]

  return (
    <ScreenBox isGrid>
      <Card gridArea="1 / 1 / 2 / 2" isCentered title="Quarantine">
        <MdVerifiedUser color="#006633" size={96} />
      </Card>

      <Card actions={scanActions} gridArea="1 / 2 / 3 / 4" isLoading={!fileExplorerRootPaths} title="Scan">
        <FileExplorer
          onChange={onFileExporerChange}
          onExpand={onFileExporerExpand}
          rootPaths={fileExplorerRootPaths ?? []}
        />
      </Card>
    </ScreenBox>
  )
}

const ScanningCancelButton = styled.button`
  appearance: none;
  background-color: transparent;
  border: 0;
  border-radius: 0;
  cursor: pointer;
  font: inherit;
  line-height: 1;
  padding: 0;

  color: rgba(255, 255, 255, 0.1);
  height: 32px;
  position: absolute;
  right: 32px;
  top: 28px;

  * {
    cursor: pointer;
  }
`

const ScanningStepText = styled.p`
  margin-top: 48px;
`

const ScanningTargetText = styled.p`
  font-family: 'Reddit Mono', monospace;
  font-size: 75%;
  margin-top: 16px;
`
