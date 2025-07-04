import { Card } from '@components/Card'
import type { CardAction } from '@components/Card/types'
import { FileExplorer } from '@components/FileExplorer'
import type { FileManager } from '@core/FileManager/types'
import { Scanner } from '@core/Scanner/types'
import { ScanningSpinner } from '@elements/ScanningSpinner'
import { ScreenBox } from '@layouts/ScreenBox'
import numeral from 'numeral'
import { MdClose, MdVerifiedUser } from 'react-icons/md'
import styled from 'styled-components'
import type { Promisable } from 'type-fest'
import { shrinkPath } from './utils'

export type ScannerScreenComponentProps = Readonly<{
  canScan: boolean
  fileExplorerRootPaths: FileManager.FilePath[] | undefined
  onFileExporerChange: (selectedPaths: string[]) => Promisable<void>
  onFileExporerExpand: (expandedPath: string) => Promise<FileManager.FilePath[]>
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
  if (scannerState && scannerState.step !== Scanner.ScannerStatusStep.Idle) {
    return (
      <ScreenBox isCentered>
        <ScanningCancelButton onClick={onScanStop}>
          <MdClose size={32} />
        </ScanningCancelButton>

        <ScanningSpinner size={128} />
        <ScanningStepText>
          {Scanner.SCANNER_STATUS_STEP_LABEL[scannerState.step]}

          {scannerState.progress && <> ({numeral(scannerState.progress).format('0.00%')})</>}
        </ScanningStepText>
        {scannerState.current_path && <ScanningTargetText>{shrinkPath(scannerState.current_path)}</ScanningTargetText>}
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
