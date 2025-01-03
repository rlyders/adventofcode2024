package day1

import "testing"

func Test_getSumOfDistancesOfListsText(t *testing.T) {
	type args struct {
		lists string
	}
	tests := []struct {
		name    string
		args    args
		want    int
		wantErr bool
	}{
		{name: "",
			args: args{lists: `3   4
4   3
2   5
1   3
3   9
3   3`},
			want:    11,
			wantErr: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := getSumOfDistancesOfListsText(tt.args.lists)
			if (err != nil) != tt.wantErr {
				t.Errorf("getSumOfDistancesOfListsText() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("getSumOfDistancesOfListsText() = %v, want %v", got, tt.want)
			}
		})
	}
}
