{{/*
Expand the name of the chart.
*/}}
{{- define "shoebox.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "shoebox.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "shoebox.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "shoebox.labels" -}}
helm.sh/chart: {{ include "shoebox.chart" . }}
{{ include "shoebox.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "shoebox.selectorLabels" -}}
app.kubernetes.io/name: {{ include "shoebox.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "shoebox.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "shoebox.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Convert structured mediaSourcePaths to string format
*/}}
{{- define "shoebox.mediaSourcePathsString" -}}
{{- $paths := list -}}
{{- range .Values.config.mediaSourcePaths.sources -}}
  {{- $path := printf "%s:%s" .name .path -}}
  {{- if .originalPath -}}
    {{- $path = printf "%s;%s" $path .originalPath -}}
    {{- if .originalExtension -}}
      {{- $path = printf "%s;%s" $path .originalExtension -}}
    {{- end -}}
  {{- end -}}
  {{- $paths = append $paths $path -}}
{{- end -}}
{{- join "," $paths -}}
{{- end }}
